# 0108: A refactoring session before stage 1

Status: settled by the architect on Nov 14, 2026, an engineering choice with no effect on play. The designer approved holding a bounded refactoring session after the cover build (decision 0106) and before stage 1 of step 3 is built. It changes no rule and no result. Not built.

Summary: A refactoring session before stage 1: one command that remakes every generated file; a comparison of every scenario against the previous build; one table describing each kind of event; the replay export carrying the state so the viewer only draws it; the ground combat file split; the decisions index made from the records. No rule and no result changes

Standing: Settled by the architect, Nov 14, 2026, with the designer's approval to hold it. Not built

## Why now

Ten rules were built in a day. Each took an hour or more, and the last team timed where its hour and three quarters went (decision 0106, the build's report):

| Phase | Minutes |
| --- | --- |
| Reading the records, the rules and the code | 25 |
| The rule's logic | 15 |
| The fan-out for new state and events: hash, loader, who sees what, export, viewer, agent | 15 |
| The rulebook and other documents | 15 |
| Builds and the test suite, three full runs | 15 |
| The scenario and its test | 10 |
| Remaking generated files, in eight manual steps | 10 |
| The slice's batch and its report | 10 |

The rule itself is an eighth of it. Most of the rest is the price of things that have worked and are kept: every hand calculation has matched, leaks have been caught by tests, and the rulebook has stayed true. But four things are faults, each of which cost time or broke a build on that day, and stage 1 would make each worse, since it adds a crate, a block of saved state, commands, tools and events.

## The bar

1. **No rule and no result changes.** Before any change, a snapshot is taken on main of every scenario file, live and frozen: the run with its events, the hash, the hash on a second seed, and a batch of 40 runs for each battle. After every pull request of the session the same commands give the same files, byte for byte. The one exception is the replay export, whose format item 4 changes on purpose; for it, the viewer's check passes on every scenario and the events in the file are the same.
2. **An item that cannot meet the bar is dropped** and reported, not forced.
3. **Nothing of how rules are decided, proved and written down changes.** The hand calculation, the decision record, the rulebook in the same pull request and the leak tests stay as they are.

## What is done

Each is its own pull request, in this order, so that each is small enough to read whole.

4. **One command that remakes every generated file.** Today that is eight steps, some by hand: a loop for the run baselines, a variable for the agent's golden briefings, a script for the recorded games, two pinned hashes edited by hand, the old batch fixture patched by script, version strings by search and replace, and two lists edited by hand. They become one command, documented in `docs/commands.md`, that remakes all of them and says what changed in plain words. The pinned hashes move into a file the command writes. The old-format batch fixture, which no tool has written since step 1, is replaced by one a tool writes, or dropped if nothing it proves is not proved elsewhere.
5. **A comparison of every scenario against the previous build,** as a command: it runs every scenario on the frozen and the live figures, strips the hashes, and reports each scenario whose events differ from the committed baselines, with the first difference. Every rule build has done this by hand, by keeping an old binary.
6. **One table that describes each kind of event.** Today a new event touches five or more exhaustive lists across four crates: who may see it, its code in the export for the log and again for each side's copy, its words in the briefing, its handling in the viewer. One of those pairs disagreed on Nov 14 and broke a build. Its code and what a side's copy is called come from one place, so that the two cannot differ. A new event still does not compile until someone has decided who sees it: that rule stays.
7. **The replay export carries the state at each step,** and the viewer draws it. Today the viewer works the state out again from the events, in its own code, which is a second copy of the rules: two rule builds had to teach it new events before its check would pass. The replay file's version rises. The viewer still reads older files, as it does now.
8. **The ground combat file is split.** It holds the fight, fatigue and posture, defeat and falling back, the day's activity and the choices a side can ask, in some 2,000 lines, and it is why the ground rules were built one after another and not side by side. It becomes a module of several files along those lines, with no change to what any function does.
9. **The decisions index is made from the records.** Every record adds a row to one table, and on Nov 14 that table conflicted four times between pull requests that had nothing else in common. Each record's first lines carry its title, its one-line summary and its standing; the index is generated from them by a command, checked in CI, and never edited by hand.

## What is not done

- **No change to the hash, to what is hashed, or to the state's layout.**
- **No change to the test suite's content.** Making it faster is not in scope unless the first item's work shows a cheap gain; if so it is reported and proposed, not done.
- **No new crate.** `sim-staff` comes with stage 1.
- **No rewrite of the rulebook or the engineering pages** beyond the lines these items make untrue.

## How it is run

One Opus lead with Sonnet developers, one pull request for each item, each read by the architect before it merges. Items 4, 5 and 9 touch tools and documents and may run side by side. Items 6, 7 and 8 touch the engine's files and run one after another. The session ends with a short report: for each item what was done, what was dropped and why, and the time one rule build's generated files and fan-out now take.

## What it costs

A few hours of team time, and one remaking of the generated files where item 4 moves them. Stage 1 starts after it.
