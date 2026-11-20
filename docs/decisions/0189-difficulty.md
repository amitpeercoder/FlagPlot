# 0189: Difficulty

Status: a plan by the architect, written on Nov 20, 2026, at the designer's asking. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A person may want a game easier or harder as he learns. Difficulty never touches the simulation: no ship, aircraft or die is changed by it, and the opponent is told nothing he would not know. It changes who the person faces and how boldly that opponent plays, and later how much his own staff helps him and how he is judged. The first step is small: where a scenario offers it, the new game screen asks easy, balanced or hard, each a way the engine's side is given its orders, and a label is offered only where a run has shown the opponent it names is the weaker or the stronger.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

## Why

The designer, on Nov 20, 2026: "a player may want to make things easier, balanced or hard as they progress." The architect set out five means and what he would not do; the designer said to plan it.

What is so today:

- **The new game screen** (decision 0185) asks the scenario, the seat and who plays the other side: the engine or an agent. Each game it offers is a file named in `table/scenarios.json`.
- **The engine's side** is a script of unit orders or a staff carrying out orders for it, written in the scenario's file. A staff's orders carry a risk: preserve the force, balanced or press, which sets when its groups rest, how far they reach and when a force breaks off. Where none is set it is balanced.
- **An agent** plays as well as its model. Seven games between two models (not in the repository) showed the stronger doing better on the same side.
- **A scenario for play** has a band read on staff against staff at balanced (decision 0187). Nothing says how a person fares against either.
- **The table shows a person no points** (decision 0157): at the end he is told the outcome in words.

## The rules

1. **Difficulty never changes the simulation.** No figure of a ship, an aircraft, a weapon, a unit or a rule, no chance, and no draw differs with it. The engine's side knows what its side would know and nothing more.
2. **Difficulty is the person's choice for a game, not a property of the scenario.** It is chosen on the new game screen, recorded with the game, and shown at its end. Games at different settings are never compared with each other, with the band or with the record.
3. **The means, in the order they are built:**
   1. **How boldly the engine's side plays:** the risk on the orders its staff is given.
   2. **Who the opponent is:** the engine's staff, or an agent, whose strength is its model's.
   3. **How much the person's own staff helps:** whether it proposes plans, warns before a force runs short, offers dry runs.
   4. **How the person is judged:** the margin that counts as a victory for him.
   5. **The situation:** a version of the scenario with another start. Each is a scenario file of its own with its own band.
4. **Not means, ever:** telling the engine's side or an agent anything of the person's forces that its side would not know; changing a chance or a figure; changing what the person is told of his own forces' state.

### The first step: easy, balanced and hard against the engine's staff

5. **Three settings,** each a file of the scenario in which the engine's side is given the same orders at a different risk: at "preserve the force" on every order, as written (balanced), and at "press" on every order. Nothing else differs between the three files.
6. **A setting is offered only where it has been shown to be what it says.** Before a scenario's settings are listed, 1,000 games of staff against staff are run with the one side at each of the three risks and the other at balanced, for each side. A risk is listed as "easy" for the person only if the engine's side does worse at it than at balanced, and as "hard" only if it does better: by the number of games in which that side is ahead, and by at least 50 games of the 1,000. Where a risk shows no such difference it is not offered under that name. The runs are reported with the scenario.
7. **This may leave no "hard" against the staff.** Pressing is bolder and not known to be stronger. Then the screen's harder game is the one it already has: an agent as the opponent. The screen says of an agent that it plays as well as the model behind it and that the game makes no claim of how hard that is.
8. **The screen** shows the choice after the seat and the opponent, only for a scenario and an opponent where more than one setting is listed, with balanced chosen first. A scenario with no settings listed is as today.
9. **The list of scenarios** (`table/scenarios.json`) gains, on a pairing against the engine's staff, its setting. Where a scenario's files are made by a script, the script makes the settings' files.
10. **Recorded and shown.** The bridge writes the setting with the game it keeps. The end screen names it beside the outcome. A game with no setting chosen says nothing of one.
11. **Scenario 103 is the first** to have its settings measured and listed. The 39 days and the one-day Eastern Solomons, whose files are proofs' and kept by hand, gain none by this plan.

### Later steps, each with a record of its own

12. **The staff's help** (means 3) is a design of the table's: what the staff offers unasked at each setting. It waits for the table's work on the staff's proposals.
13. **How the person is judged** (means 4): since he is shown no points, this is the word he is given at the end. A margin of his own for each setting would be set with a reason, as levels are.
14. **Another start** (means 5) follows decision 0187 in full: a file of its own, for play, with its own band.

## Rule card

No rule of the simulation is added or changed. New: scenario files that differ in the risk of the orders given; a key on a pairing in the table's list; a choice on the screen; a line kept with a game.

## What it costs

- **Files:** two more for each side of each scenario that offers settings, made by its script.
- **Runs:** six of 1,000 games for a scenario, a few hours of machine time, before its settings are listed, and again when its file or a doctrine it uses changes.
- **A label a person may not feel.** The measure is staff against staff. A person may find "easy" no easier. The record says what was measured; the screen makes no claim beyond the word.
- **Documents:** decision 0185's screen; the table's list and its note; `docs/commands.md` for the bridge.

## Questions for the designer

1. **The words:** easy, balanced and hard; or the order's own words (cautious, balanced, bold), which claim less.
2. **If pressing proves no stronger,** is "hard" against the staff left out (the architect's proposal), or should a stronger staff be built for it later.

## Amended on review

A reviewer who had not seen the plan written read it against the staff's rules, the screen's code and the records. Fifteen findings; the architect's ruling on each is here. Found right: what the screen asks today; that a scenario file can set a risk on each order on day 1; that files differing only in that are possible; that no rule of the simulation changes.

### The words

15. **No setting is called "balanced".** Decision 0185, rule 12, says the screen claims no balance, and decision 0187 says scenario 103 is not to be called balanced until its games between agents are played. The three settings are **easier, standard and harder**. Standard is the scenario as its orders are written. Showing a setting is not a claim that any of them is a fair game, and the record of each scenario's settings says what was measured.
16. **A label follows the measurement and not the risk.** From the rules, pressing may be the weaker way to play: a Japanese staff at press sends the convoy by day where at balanced it goes by night, and an Allied staff at preserve strikes no destroyer or transport from Lunga. So "bold" may be the easier opponent. The designer's question about the order's own words (cautious, balanced, bold) is withdrawn: those words could name the easy game "bold".

### The measurement (replaces rule 6)

17. **The files measured** are versions of the scenario's file in which both sides are given orders, with one side's orders at preserve or at press and the other's as written: four files for a scenario, made by its script into a folder of their own. They are never the band's file (decision 0187, rule 23) and are never offered for play.
18. **Five runs,** not six: the scenario as written, and the four. All on the same commit, on seeds 1 to 1,000, each seed the same game until the orders make it differ.
19. **Read in pairs, seed by seed.** A side is ahead in a game when the margin is on its side; a margin of 0 is for neither. For one side at one risk against the same side as written: b is the seeds in which that side is ahead as written and not at the risk, c the seeds the other way. That side at that risk is **weaker** if b - c is at least 50 and at least twice the square root of b + c; **stronger** if c - b is. Otherwise it is no different. The report gives b and c beside the totals, the commit and the staff's doctrine versions.
20. **From weaker and stronger to the list.** For the person's opponent on a side: if one risk is weaker it is listed as easier; if both are, the weaker of the two (the larger b - c). If one is stronger it is listed as harder; if both are, the stronger. A risk that is no different is not listed. If neither is listed for a side, that side's games show no choice.
21. **This may list nothing.** The nearest run on record (bold commanders in the one-day Eastern Solomons, stage 2b's proof) changed no margin on any seed. If the five runs show no difference for either side, what is built is the mechanism and the report, the screen shows no choice for scenario 103, and the next means (the opponent, the staff's help) is where difficulty is looked for. That is reported to the designer and nothing is adjusted to make a setting appear.
22. **Measured again** when the scenario's file or a doctrine it uses changes; `docs/open-work.md` keeps it owed, as it does the band.

### What the person learns, and what the opponent does not

23. **A person who chooses a setting knows something of how the enemy has been told to play.** That is the price of choosing and is accepted. He is not told which risk a setting is: the page is given the word (easier, standard, harder) and nothing else, the list the bridge sends the page carries no risk, and no file name the person sees names it.
24. **An agent is told nothing of a setting,** in its brief, at its door or anywhere. In this step settings are offered only where a person plays the engine's staff; a watched game of an agent against the engine has none.
25. **Rule 2, said exactly:** the results of games at different settings are never laid beside each other for the record or shown to a person as a comparison. The measurement of rules 17 to 20 compares them, and is the umpire's.

### The screen and the bridge

26. **Scenario 103 is first put on the table's list,** which it is not today: its files for a person against the engine's staff on each side, a person against an agent, and the watched games. Being listed claims no balance.
27. **A game is found by who holds each side and by its setting.** Today the bridge and the page each take the first pairing that matches the holders. Both are changed to match the setting too, in beginning a game and in the page's choice; the bridge refuses a list with two pairings of the same holders and setting; the screen's check script, which today pins the number of games offered and begins each by scenario and holders, is changed with them.
28. **Kept with the game:** a line in the game's own folder, where the bridge keeps games, and in what the bridge tells the page of the game in hand. The end screen names the setting on the line that says the days fought. A game with no setting says nothing.

### The orders and the hand calculation

29. **Each setting's orders are written in the scenario's hand calculation** by the architect before the build: the same orders, with the risk on each. Scenario 103's says today "No risk is set: every order is at balanced", names four files and tests for four; the architect changes those lines and the expected count, and names the new files, before the build is delegated.
30. **A setting is not a means of balancing** in decision 0187's sense. The band is read on the scenario as written and no setting's file is used to bring a scenario into it.

### What each risk changes, for the record

31. **The risk reaches every order,** read up the tree. It changes: for a carrier force, its patrol share, how often it searches, when it rests, its fuel margin, when it breaks off, its speed, and what it closes on; for a raid or a strike on ships, when the group rests, how old a contact it will fly at, which ships it will strike, its reach, and what it holds back; for a hold, the supply floor, whether the force it sends goes by night, and its cover. It changes nothing in a hold's orders to its men, in the forces an order may use, or in what a neutralise or a deny forms.

### The means, added to

32. **Under the staff's help (means 3):** how often the clock stops for the person and which interrupts are on when a game begins. They are the person's own aids and touch nothing of the simulation.
33. **Scenario 105's script** (the Coral Sea, decision 0188) is written ready for settings.

### The questions, as they now stand

1. The words are easier, standard and harder, unless the designer prefers others that claim no balance.
2. If the runs list nothing against the staff, difficulty is looked for next in the opponent and in the staff's help. The architect proposes that order.
