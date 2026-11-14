# 0085: What a side is told of a fight, and other leaks an outside review found

Status: decided by the architect on the designer's delegation of Nov 13, 2026 ("Go ahead and decide all matters that need decisions"). It amends decision 0047 (items 5, 7 and 9, and its table of kinds) and decision 0061's rule that breakdowns follow the event.

Summary: What a side is told of a fight, and other leaks an outside review found: a fight's ending is told as the side's own force made it, else "contact lost"; a fight is told from the round the side first knew of it, its hour and rounds counted from there; hidden causes are flagged by kind; no raw draw in a side's breakdown; `strike_choice` by number; no saved game to the player before the end; a day that cannot be resolved gets one fixed answer, and the loader refuses an arrival that overfills a ship. The escort shots and the claims are ruled unchanged; numbering per side is left to 0086. Built (Nov 13, 2026)

Standing: Decided by the architect on the designer's delegation

## The gap

On Nov 13, 2026 a reviewer from outside the project was asked whether a side can learn what it should not. Pull request 213 reproduced each of its eight findings as a test, switched off, and looked for more. Every finding was true. Some reach a player through the served tools today. Others reach only a Rust caller holding a `SideView`, which the staff planner of step 3 will be. Two of the review's findings, and one of the team's own, change numbers that random draws are keyed on, so they change runs; they are left to decision 0086 (below). The rest change only what a side is given, and are decided here.

## Decisions

1. **A side is told how a fight ended only as far as its own force made the ending.** In the side's form, the ending "a force was sunk" is kept only if the side's own force was the one that lost its last ship, and "a force broke off" only if its own force broke off in the last round. Otherwise the side is told one neutral reason, **contact lost**. The enemy's losses still reach it only as claims, and its break-off not at all. Slipped away, passed unseen and the last round fought are told as before. A side's form of the ending has a type of its own, `SideEndReason`, so the enemy's reason cannot be carried in it. (Finding 5, and the same leak through "broke off".)
2. **A side's view of a fight begins at the round it first knew of it.** Its `EngagementBegun` carries the hour of that round (the meeting hour plus `round_minutes` for each round before it), not the hour the forces met, and every round it is told is counted from 1 at that round. A side that knew from round 1 is told what it was told before. This is the one alteration made to an event given whole: a hit, a sinking, an alert or a break-off inside the fight carries the side's own count of the round. It replaces the clause of 0047 item 7 that nothing is altered, for rounds only. (Finding 6.)
3. **Whether an event may have causes the side cannot see is said by its kind, not by the event.** One exhaustive `match` over the kinds a side is given says, for each, whether any cause it can carry may be an enemy order or an event the side is not given. Causes are dropped exactly as before; only the flag changes. An enemy order that changes nothing now changes nothing the side is given. It amends 0047 item 9: the flag means "may have more", and is set for every event of such a kind. (Finding 7.)
4. **A side's breakdown never carries a raw random draw.** The alarm roll recorded the die with its chance; the side's form drops any factor named `draw`. The umpire's breakdown keeps it. (The team's own finding C.)
5. **`SideView::strike_choice` takes an air group's number,** and looks it up among the side's own groups. A caller can no longer hand the view a group of its own making aboard an enemy ship and read that ship's true position from the answer. `TruthState::air_group_hex` stays public: tests in other crates read it, and it is not on the view. (Finding 1.)
6. **A saved game is given to the player only when the game is over.** `play --export-save` refuses an unfinished game, with the reason, and writes nothing. A saved game holds both sides' orders: it is the umpire's (decision 0050, 11). `--export-transcript` alone is still given mid-game: it holds only the seat's own calls. `serve --save` and `mcp --save` are the operator's flags and are unchanged. That a saved game records the script's orders as issued by the player is listed in `docs/rules/differences.md`, row 40, and left until the recorded games are remade. (Finding 2.)
7. **A day that cannot be resolved is answered with one fixed sentence.** The engine's own words, which can name any unit, go to standard error for the operator. And the loader refuses an arrival that would put more aircraft aboard a ship than its class carries, counting the aircraft aboard at the start and every arrival to the ship's groups up to that day, losses ignored. This carries out row 27 of `docs/rules/differences.md`. (Finding 4.)
8. **Ruled, no change: the fighters' shots at escorts.** A defending patrol fires at the escorts with at most as many fighters as there are escorts, so its own shot count can tell it how many escorts there were. The fighters met the escorts; what they fired is their own (0047 item 3). It stays.
9. **Ruled, no change: claims.** With no ship burning, a surface claim of ships sunk equals the truth, against what the briefing promises (claims "can be well above" it). That is the claim rule of 0014 and 0056, a question of figures and not of the boundary. It goes on the list for the claims round of tuning.
10. **Left to decision 0086: numbering per side.** A task force or air group made during a game takes the next number after both sides' highest; the scenario files number both sides' units from one range; and a `SideView` hands a Rust caller the log's event numbers and order numbers, which count both sides. Each tells a side something of the enemy's numbers, and the first changes random draws. They are closed together, with per-side numbering, once the pull requests that change state ahead of it have landed. The tests that show them stay switched off until then.

## Why

- **The ending and the start are what a side would not see.** A side that loses touch with the enemy does not know whether it sank or slipped away; that is what the claim is for. A side that first sights the enemy in what the umpire calls round 3 does not know two rounds went before.
- **A flag by kind costs a little honesty and closes the leak.** "May have more" on an event that has none is true as a "may". A flag that is on exactly when something was dropped tells the side that something happened.
- **By number, not by value.** The view cannot check that a value it is handed is the truth; it can look a number up among the side's own.

## What it costs

- **A fight's end is less specific.** A player who sank the enemy's force reads "contact lost" and must read its claim. That is the intent.
- **Round numbers differ between sides.** The umpire's round 3 is a late side's round 1. The event numbers still match, so two sides and the umpire can still talk about one event.
- **An emptied force is judged from the state as it is now.** A force that lost its last ship is never refilled today, so "own force sunk" is read as: the force has no ship now, and its last sinking was in this fight. If a rule ever refills an emptied force this must be looked at again. One case reads wrongly and is accepted: a force that loses a ship in a fight and then detaches all its other ships in the same fight is told "a force was sunk". It is told so of its own force only, and a side knows its own units.

## Built

All of the above (Nov 13, 2026). No run, state or hash changes: the run baselines, the pinned hashes and the recorded games' saved games are unchanged. The tests of pull request 213 for findings 1, 2, 4, 5, 6 and 7 now pass and are switched on; those for 3 and 8 stay switched off for decision 0086. A new test checks over eight scenarios that every event that lost a cause is of a kind flagged as having hidden causes.

## Rule card

| | |
| --- | --- |
| Reads | The event log; which side each unit belongs to; which task forces have no ship; the surface combat figure `round_minutes` |
| Writes | Nothing |
| Events | None |
| Randomness | None |
