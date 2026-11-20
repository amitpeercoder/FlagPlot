# 0187: A scenario's scoring is its own

Status: a plan by the architect, written on Nov 20, 2026, on the designer's ruling of that day. It changes decision 0054 (one scale for every scenario) and decision 0184, rule 17 (points are not fitted to a run). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ).

Summary: The figures of the simulation are one set for every battle and stay so. What a side is scored for, and how much, is now each scenario's own. A scenario meant to be played against someone may be given objectives, points and levels that make it a contest, under three guards: the reason is written before the number, adjusting stops inside a stated band, and games between agents are played before it is called balanced. Scenarios that check the simulation against the record keep the common scale.

Standing: In force from Nov 20, 2026 on the designer's ruling, amended on review; its one piece of code, the key that marks a scenario for play, is built with decision 0186.

## Why

The designer, on Nov 20, 2026: "As a whole, it may be the case that different scenarios have different scoring for the sake of balance." The architect proposed the rules below and he said to go ahead.

Scenario 103 (decision 0184) showed the need. On the common scale the Allied side won 846 and then 963 of 1,000 games of staff against staff. The forces and the orders were the battle's; what made it no contest was that two of the three Japanese aims could not score. History is seldom even. A scenario's result should say who did better than his situation allowed, and a count of damage on one scale for every battle does not say that.

What is so today:

- **Decision 0054** gives the game one placeholder scale, so that "a run, a batch of a thousand runs and the record" can be laid side by side.
- **Decision 0184, rule 17,** holds that fitting a scenario's points to a run "is balancing, and the project's rules do not allow it as written", and leaves a scale of a scenario's own to the designer's ruling. This is that ruling, made general.
- **The rules of tuning** (`docs/step2-spec.md`) are about the simulation's figures: one set serves every battle, and none is set for one scenario. They say nothing of points.
- **A scenario already carries its own objectives, points, levels and horizon** in its file. Nothing in the code holds them to one scale. This decision changes what a scenario may write, not what the game can read.

## The rules

1. **The simulation's figures are common.** Nothing here touches them. No ship, aircraft, weapon, unit or rule is given a different figure in one scenario for the sake of its balance.
2. **A scenario's scoring is its own:** which objectives each side has, what each is worth, the three levels and the horizon.
3. **Two kinds of scenario,** and each scenario's header says which it is:
   - **a scenario of record** checks the simulation against history or proves a rule. It keeps the common scale of decision 0054, lopsided or not, so that its runs can be laid beside the record and beside each other. The battles of step 2, the proofs of step 3 and the 39 days are scenarios of record. The 39 days' scale is still to be set from the record before stage 5 of step 3;
   - **a scenario for play** is meant to be played against a person, an agent or the engine. It may have a scale of its own. Its results are never compared with the record of the battle or with another scenario's.
4. **The means of balancing a scenario for play, in the order they are tried:**
   1. **Score what each side was sent to do.** An objective that comes from the mission, of a kind the game has or a new kind with its own decision record.
   2. **Weigh the points for this scenario.** What a carrier or a base is worth here.
   3. **Move the levels.** How large a margin is a victory here.
   4. **Change the forces or where they start.** Last, and only on the designer's word, as decision 0184, rule 20, has it: it stops being the battle.
5. **Guard one, the reason before the number.** Every objective and every figure of a scenario's own scale is written in the scenario's decision record with its reason, in terms of the mission or the history, before a run made with it is read. A figure whose only reason is that a run came out even is not written. Where the architect has seen a run before choosing, the record says so.
6. **Guard two, a band and not a point.** A scenario for play is a contest when, over 1,000 games of staff against staff on seeds 1 to 1,000, each side given the orders its mission implies, neither side wins more than 650. Inside the band nothing more is adjusted. Outside it, the next means of rule 4 is tried, once, with its reason; each change is recorded beside the run before it and the run after. After two changes that leave it outside the band it goes to the designer with the counts.
7. **Guard three, agents and not only staffs.** Before a scenario for play is called balanced, four games between agents are played, two with each of two models on each side, and their margins are reported beside the staffs'. They set no figure. If the agents' games all go one way where the staffs' are in the band, that is told to the designer.
8. **What the band is not.** It is a test that a scenario is a contest for the engine's staffs. It does not say the scenario is fair between two people, and the record says so wherever it gives the band.
9. **A history note stays a note.** A scenario may say what happened in the real battle. Scoring a side against the historical outcome (the real result as the mark to beat) would be a new rule and waits for a record of its own.

## Rule card

- **Reads:** nothing new.
- **Writes:** nothing. It is a rule about what a scenario's file may say and how its figures are chosen.
- **New:** a line in a scenario's header saying whether it is of record or for play. No code reads it.

## Scenario 103, the first set this way

10. **It is a scenario for play.**
11. **First means, an objective from the mission:** men landed at Taivu (decision 0186). The Japanese mission is to land the convoy's men; the game scored only its supply.
12. **Its figure, 1 point for each 100 men,** for the reason decision 0186 gives: a man put ashore is worth what the enemy is scored for destroying him. The architect had seen the second run before choosing and says so (guard one).
13. **The band.** From the second run's counts, with no game replayed since no game changes: Allied 483, Japanese 249, stalemates 268. Neither side is above 650: inside the band, and nothing more is adjusted. "Lunga shut" stays at 10 points; it is a Japanese aim of the mission, seldom got.
14. **Still owed** before it is called balanced: the four games between agents (guard three).

## What it costs

- **Runs of different scenarios can no longer be laid side by side** unless both are of record. That is the price, and rule 3 keeps the comparison where it is used.
- **A scenario for play can be fitted to the staffs** and still be one-sided between people. Guards two and three limit that and do not remove it.
- **Documents:** the rulebook's chapter 6 says the scale is each scenario's and names the two kinds; the glossary gains "scenario of record" and "scenario for play"; `docs/engineering/roles-and-tests.md` gains the three guards where it speaks of building a scenario; decision 0054 and decision 0184 each gain a line pointing here.
- **No code changes.**

## Amended on review

A reviewer who had not seen the plan written read it against the records and the code. Thirteen findings; the architect's ruling on each is here. The claims about decisions 0054 and 0184, and that no code or test holds scenarios to one scale, were checked and found right.

### What was wrong

15. **The band could always be met by raising the levels,** since a win was a margin at or above the minor level and a higher level turns wins into stalemates. Rule 6's band and rule 4's third means are replaced:
    - **The band is read on the sign of the margin, which no level moves.** Over the 1,000 games, a side is ahead in a game when the margin is on its side, by however little. A scenario for play is a contest when neither side is ahead in more than 650 of the 1,000.
    - **The levels are not a means of balancing.** They are set for the scenario's size with a reason, before a run is read, and are not changed to enter the band. The run's report gives how many games reach each level and how many are stalemates; that is reported and not judged.
16. **There is no one common scale to keep.** Decision 0054 prices ships and aircraft for a one-day battle; the kinds added since have no figure there, and scenarios that prove a rule already carry levels and points of their own (scenario 16, 23, 32 and 43 among them). Rule 3's first part is replaced: **a scenario of record keeps the scoring it has. Its scoring is never chosen or changed for balance.** What its scale should be is decision 0054's and step 2's question (the scale set from the record), which this plan leaves where it is.
17. **A line in every scenario's header would break pinned files** (the digest of scenario 5's files in scenario 103's test, the frozen copies, the files the step 2 reports name by hash). Rule 3's "each scenario's header says which it is" is replaced: **a scenario is of record unless its file says it is for play.** No existing file changes.
18. **"No code changes" was wrong.** The kind is carried as data (rule 22), and `sim-cli` prints "the placeholder scale of decision 0054" over every result, which is false for a scenario for play.

### What was missing

19. **A scenario for play is never the scenario of a proof or of a validation measure,** and a proof's scenario is never made for play. A version for play of a scenario of record is a file of its own. The 39 days as the table offers it today is the proof's file, of record, lopsided as it is: a learning scenario. A balanced 39 days would be another file under these rules.
20. **What is not a means of balancing, and is not changed to enter the band:**
    - the orders a side the engine plays is given. They are what the mission implies, written in the hand calculation. They may be mended where they leave out something the mission asks for, with the reason written before the run, as decision 0184's second version was; that is not one of the means, and an order moved because of what a run showed is a figure set under another name (`docs/step2-spec.md`, the rules of tuning, 8);
    - the horizon, which is set from the length of the operation (decision 0088);
    - the scenario's length, its starting supply, its placeholder figures and the rules it switches on.
    The means are then three: an objective from the mission; the points of this scenario; and, on the designer's word alone, the forces or where they start.
21. **Guard one is made a test and not only a disclosure.** A figure chosen after a run has been seen must be either a figure another objective of the game already has, used by a stated likeness, or one that follows from the mission by a stated rule. A figure with neither is not written, whatever the band says.
22. **The kind is data.** A scenario for play writes `for_play = true` in its `[objectives]` table; absent, it is of record. With it:
    - a result is printed as on "the scenario's own scale, for play" in place of the words about decision 0054;
    - the batch file's header carries it;
    - `sim-cli validate` refuses a scenario for play, since its tally is not for the record.
    It changes no game and no hash. It is built with decision 0186.
23. **Guard two, defined.** The run is of the scenario's file in which both sides are given their orders, seeds 1 to 1,000. Its report names the commit and the staff's doctrine versions. A change is one means applied once with one reason, however many figures it sets. The band is read again when the scenario's file or a doctrine it uses changes; `docs/open-work.md` keeps that owed. Where no game changes (an objective or a figure added, which nothing in play reads), the band may be read from the kept counts of the last run, and the record says that it was.
24. **Guard three, defined.** Four games of agent against agent on the scenario's file in which neither side is given orders, with two models, X and Y: X as the first side against Y twice, and Y as the first side against X twice.
25. **The records this touches, beside those named:** `docs/step3-spec.md` where it says "on one scale"; chapter 6's note on the placeholder scale; the glossary's "level"; `docs/open-work.md`; scenario 103's hand calculation where it says "the game's one scale", which the architect changes; scenario 103's base and its script. The designer's ruling in `docs/step2-spec.md` that the scale is set from the record stands for scenarios of record.

### Scenario 103, read again

26. **Its file says it is for play.**
27. **The figure passes guard one as now written:** 1 point for each 100 men is the figure men lost already has, used by a stated likeness.
28. **The band, on the sign of the margin,** from the second run's kept counts, no game replayed: the Allied side is ahead in 615 games and the Japanese in 385. Inside the band, by 35 games. Without the objective it was 984 and 16; at a half point, 884 and 116.
29. **What that balance is, said plainly.** In the staffs' games the convoy lands in 997 of 1,000, so the objective is close to a flat 15 points to the Japanese side there. Against a person or an agent who strikes the convoy it is not flat: it is what the convoy is worth. The middle game is a stalemate (a margin of 2.51 against a minor level of 3), and 268 of the 1,000 are. The band says the staffs' games do not go one way; it does not say the scenario is a good game. The four games between agents are owed before it is called balanced.
30. **The levels stay 3, 8 and 16.** They were set before any run and are not moved.

## As built

Rule 22 was built on Nov 20, 2026, with decision 0186, in one pull request.

- **The key.** `for_play = true` in a scenario's `[objectives]` table loads into `Objectives::for_play`; absent or `false`, the scenario is of record. Only scenario 103's four files and their base have it.
- **`sim-cli run`** prints the result of a scenario for play under "Result (the scenario's own scale, for play):", and of every other under the words it had.
- **The batch file** has `"for_play"` in its header, always written, after `label` and before `runs`. Following `daily_supply_t_per_man`, the version stays 6; a file without the key is of record.
- **`sim-cli validate`** refuses a batch whose header says for play, and one whose scenario on disk says so, before it writes anything: "FILE is a scenario for play (decision 0187): its results are not for the record, so validate does not judge it". It exits with an error. A scenario of record is judged as before.
- **Documents.** Chapter 6 says the scale is each scenario's and names the two kinds; the glossary has "scenario of record", "scenario for play" and a new "level"; `docs/commands.md`, `docs/step3-spec.md`, `docs/engineering/roles-and-tests.md` (the three guards) and `docs/open-work.md` are changed.
- **The band, read again** from the third count of file a's 1,000 games (decision 0186, As built): the Allied side ahead in 615 and the Japanese in 385, as rule 28 said. No game changed.
- **Not done here:** scenario 103's hand calculation, where it says "the game's one scale", is the architect's (its test of the objectives is ignored until then); the four games between agents (guard three) are still owed.

## The architect's rulings on the build

Read on Nov 20, 2026. The key is built as rule 22 has it: `for_play` in a scenario's objectives, the words over a result, the batch file's header (its version unchanged, as when the daily supply figure was added), and `sim-cli validate` refusing. Scenario 103's base carries the key because it is the source its script reads; it is not played, and that stands. Scenario 103 is the first scenario for play. Its four games between agents are still owed.
