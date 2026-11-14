# 0100: Step 3, stage 0: the ground the staff stands on

Status: a plan by the architect, written on Nov 14, 2026, for the designer's approval, as `docs/step3-spec.md` asks of every stage. Decisions 4 and 8 need review. Nothing is built. It adds no rule of the game and changes no result: every existing scenario must give the events and the hashes it gives today.

## What stage 0 is

The spec gives stage 0 three things and one experiment:

- the choices the rules make for a unit inside a day, as functions a staff can ask;
- a saved game that says what rules it was played under;
- a measurement of what a day costs with a staff's workload on top;
- a small experiment in building a trial's private world, to find what is missing before stage 4 leans on it.

## What was found

The code was read for every choice a rule makes on a unit's behalf during a day. There are about twenty.

- **One is already a function a side can ask:** which contact a strike group would go for, and whether it would launch (`SideView::strike_choice`, decision 0049).
- **About a dozen read only what the side knows** (its own units, its standing orders, its belief and the figures) but are written inside the code that carries them out, so nothing can ask them without running the day. They are listed in decision 1.
- **Five read the truth about the enemy.** Each is made at the point of contact, by men who can see what the staff cannot:

  | Choice | What it reads |
  | --- | --- |
  | Which enemy force a raid attacks when it arrives, and whether it finds anything | The enemy forces truly in the hex |
  | Which ship each strike group attacks | The true ships of that force |
  | How a patrol divides between the raids over it, and between escorts and bombers | The true size of each raid |
  | Whether a force that avoids slips away | Whether the enemy has sighted it |
  | Who fires at whom in a surface fight | The enemy ships truly afloat |

- **A side's view lacks some of the figures its own choices use:** the surface combat figures (how near a contact alerts a force, when a ship counts as crippled, the escorts a straggler takes), the fatigue figures and parts of the air figures. They are rules, not secrets, and are simply not handed over.
- **A saved game records the hashes of its scenario and data files and nothing of the rules.** A change of rules shows only as a hash that differs on some day of the replay. The batch file already records the version of the state's layout; the saved game does not.
- **Nothing in the repository measures time.** The only figure recorded is the soak's: 180 days in 0.27 seconds, release build. The lint forbids reading the clock anywhere in the workspace.

## Decisions

### Choices as functions

1. **Each choice that a side makes from what it knows becomes a function of that and nothing else,** and the engine calls the same function, so there is one answer and one place it is written. They are:

   | Choice | Reads |
   | --- | --- |
   | How many aircraft of a group can fly | The group, its base's supply, its ship's flight deck, its fatigue, the day it moved |
   | Whether a base strike launches this pulse | The group, its target, its reach, the base on the map |
   | How a fighter group splits into patrol and escort | The group and its patrol share |
   | Which raid the escorts go with, or whether they stay | The side's own raids of the pulse |
   | Where aircraft land when their deck is gone | The side's carriers in the hex and its bases in reach |
   | Whether a tired group stands down or returns | The group's fatigue and threshold |
   | Whether a force is alerted | Its position and readiness, and the side's contacts |
   | Whether a force is ready when a fight begins | Its readiness and when it was alerted |
   | Whether a force breaks off | Its own ships sunk and crippled, and its rules of engagement |
   | What a force does with its stragglers | Its ships' propulsion, their kinds, its rules of engagement |
   | Where a straggler or a retiring force goes | Its home port |
   | Whether a force turns back or goes on when its route changes | Its position on its leg, and the map |
   | Whether a force halts at a marked waypoint, and when it steams on | Its route, mission, cargo order and ships, and the bases on its map |
   | Whether a land unit goes back to defend | Its posture and fatigue |
   | What a land unit's day is | Decision 0099, 3b |

2. **A side can ask each of them through its view,** as it can ask about a strike today. The answer is what the engine would do at this moment with what the side holds. It is not a forecast: it does not say what the enemy will do or what the dice will give.
3. **A side's view hands over the figures those choices use.** Figures are the rules of the game and are the same for both sides. Nothing of the enemy is added to the view.
4. **The five choices made at the point of contact stay with the rules.** They are not staff decisions: the pilot over the target and the captain in the fight see what is there, and the staff does not. A staff cannot ask them and does not need to. A trial does not ask them either: it plays them, in its own private world. *Needs review: the other course is to make them choices from belief, so that a raid attacks the kind of ship it was told of and not the ships that are there. That would change the results of every air and surface fight and undo the tuning of step 2. The architect recommends against it.*
5. **Which base feeds, refuels or repairs is not a choice.** It is a fixed rule of allocation (the lowest-numbered base of the side in the hex, in order) and stays where it is. A staff reads the rule; it does not ask it.

### The test of it

6. **Nothing changes.** After this stage every scenario gives, byte for byte, the events, the daily hashes and the baselines it gives before. The layout of the state does not change and the hash version is not raised.
7. **Ask, then run, then compare.** For every scenario in the repository and every day of it, each function is asked from each side's view before the day runs, and its answer is set beside what the engine then did. They agree, or the difference is one this record names: something that changed inside the day before the choice was made (a ship sunk that morning, a contact reported at noon). This is the stage's main test, and it is a test of every scenario at once.

### A saved game and its rules

8. **A saved game records what it was played under:** the version of the state's layout, as a batch file does, and a rules label, which this stage begins at 1 and which a pull request raises when it changes a rule in a way that changes what an existing game would do. A replay of a game whose record differs from the engine's is refused before it runs, with the two versions named. *Needs review: an older saved game is refused, with the reason, and not converted. A game in progress does not survive a change of rules, as decision 0055 already said. The other course is to carry old games forward, which would mean keeping every old rule alive.*
9. **The recorded games in the repository are remade once more** in the new format. From then on a rule change that would break them says so at once.
10. **Step 3 adds to the record as it goes:** the version of the doctrine, when there is one, and of the trial's settings.

### The measurement

11. **Time is measured by a command of `sim-cli`, outside the game.** The lint against reading the clock stays for the whole simulation; the one command that measures is exempted by name, and nothing it measures can reach a result.
12. **What it measures,** on a named machine, each as a spread over repeated runs:

    | Workload | What it stands for |
    | --- | --- |
    | One day of the full slice, early and late in the game | The cost a staff is added to |
    | That day with every function of decision 1 asked for every unit of both sides | A staff's daily decisions, at their dearest |
    | A copy of the world, early and late, with and without its event log | What a trial pays before it plays a day |
    | A hundred games of 30 days from one start | A trial of the size the spec has in mind |
    | The same on the soak at day 150 | A long game with its history behind it |

13. **Its report says what a trial could cost and what would have to change to make it cheaper,** and gives the designer the numbers to set the times the spec's "Speed" asks him to accept. The 39-day slice is measured when it exists; the 14-day slice and the soak serve until then.

### The constructor experiment

14. **A world is built from a side's view, three times,** from the slice on a quiet day, on the day of the carrier battle, and late in the game: once from each side. It is an experiment on a branch that is not merged. What is kept is its report: what a view gives, what had to be assumed, what could not be built at all, and what the view or the scenario must say for stage 4 to do it properly.

## What it leaves for later stages

- **A staff.** Nothing here decides anything new.
- **Opening both seats to agents through the game's own server.** One side is still a script there; the test server opens both.
- **Forecasting.** A function says what a unit would do now. What will happen is the trial's.

## What it costs

- **A refactor across the air, surface, movement, straggler, landing and ground code,** with no change in behaviour. It touches the files every ground rule in flight touches, so it is built when they have merged, not beside them.
- **New methods on a side's view, and the figures it hands over.** Each is checked by the leak tests: two worlds that differ only in what a side cannot see give the same answers.
- **The agent's tools gain the questions,** and the rulebook gains, for each choice, the sentence that says a side can ask it. Chapter 1 gains the line between what a commander decides and what is decided at the point of contact.
- **The saved-game format changes once,** and the recorded games are remade.

## Rule card

| | |
| --- | --- |
| Reads | What each choice reads today |
| Writes | Nothing new |

No rule is added, changed or removed. The rulebook changes to say which choices a side can ask, which is a fact about the interface, and the glossary gains "point of contact".

## Questions for the designer

1. **Decision 4:** the five choices made at the point of contact stay with the rules and read what is truly there. Agreed?
2. **Decision 8:** a saved game from before a change of rules is refused, with the reason, and is not converted. Agreed?
