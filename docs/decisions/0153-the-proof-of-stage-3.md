# 0153: The proof of stage 3: the 39 days, staff against staff, from area intents alone

Status: a plan by the architect, written on Nov 17, 2026, before step 8 of stage 3 is built and before anything of the 39 days is run with a staff. It is step 9 of decision 0139's nine. The conditions and their pass marks are that decision's, confirmed by the designer on Nov 17, 2026, and none is changed here: this record says how each is read, what each side states, and what is done before the judged run. Nothing is run.

Summary: The plan for the proof of stage 3, written before the run: the 39 days of the staff's setup played on a thousand seeds with each side given area intents alone and no forces named, in three arrangements; what each side states; how each of decision 0139's eight conditions is read from the umpire's files; the rule that picks the 24 reasons; a loading check on one seed before any judged run, with what may and may not be changed after it; and what is reported and not judged

Standing: A plan by the architect, Nov 17, 2026. Not run

## Why it is written first

A proof that is written after its run can be made to pass. The pass marks were set with the stage's plan. This record fixes the rest before any staff has played a day of the 39: the intents, the measures, and what a loading check may change.

## What the staff cannot do, and what the plan does about it

Each of these is a limit of the stage as built, named in its record. The run is made with them and the report says where each shows.

| The staff cannot | So in the run |
| --- | --- |
| Choose a beach, or march from one to a place inland (decision 0143) | Each side lands at the place itself. The Japanese landings of the record at Taivu and to the west, and the marches from them, are not played |
| Attack from a place it holds (decision 0144) | A side that holds the airfield defends it and does not sortie from it ashore |
| Refuel at sea (`docs/open-work.md`) | A force that lacks the fuel goes home. The report counts how often |
| Time a passage longer than a night to end in the dark (decision 0145) | Such a run goes by day at balanced and is reported |
| Judge what a bombardment or a raid did (decision 0146) | A bombarding force stops on its rounds |
| Move men between places it holds (decision 0152) | The crossings from Tulagi to Guadalcanal are not played |
| Bring forces together, or sail one operation from two bases (decision 0149) | Each lift is one force as the setup has it |

## The setup and the three arrangements

The staff's setup of the 39 days, `data/scenarios/staff/07_full_slice.toml` (decision 0140): no scripted order, each force with the day from which it may be used. Release build, live figures, command levels off, seeds 1 to 1,000.

| Arrangement | Allied | Japanese | It serves |
| --- | --- | --- | --- |
| (a) | intents | intents | Conditions 1 to 4, 6 and 7, and both halves of 5 |
| (b) | nothing | intents | The Allied half of condition 5 |
| (c) | intents | nothing | The Japanese half of condition 5 |

A side given nothing has no intent and no order: its staff does nothing, as every staff does without an intent.

## The islands

Two islands are added to the setup as data (decision 0151, item 1), each from the bases the slice already has, and nothing else of the setup changes for them:

| Island | Its places, in order |
| --- | --- |
| 1, Guadalcanal | Lunga (base 1) |
| 2, Tulagi | Tulagi (base 4) |

The Japanese stocks at Taivu and west of the Lunga are not named places: they are stocks the slice put where the record's landings were, and neither side is asked to take or hold them.

## The intents as stated

Each side states area intents alone, on day 1, with no forces limit, no risk set (so balanced) and both leaves given.

| Side | Intent | Command |
| --- | --- | --- |
| Allied | 1 | control island 1, Guadalcanal |
| Allied | 2 | control island 2, Tulagi |
| Japanese | 1 | control island 1, Guadalcanal |
| Japanese | 2 | control island 2, Tulagi |

Nothing else is stated. What each staff forms from them is its own doing and is what the proof is of: the takes, the holds and their supply, the denial, what is neutralised, the lifts, the cover, the bombardment and the reconnaissance.

## The loading check

Before any judged run, arrangement (a) is run on seed 1 alone, with breakdowns on, and read.

1. **What it is for:** to find a command of the table above that is refused, a place the tools cannot name, and a force or a base the setup has in a state no rule of this stage can use at all.
2. **What may be changed after it, by the architect, in writing, before any judged run:** the wording of an intent where a tool takes it otherwise; a fault in a rule, mended in its own pull request with its hand calculation; and the setup's data for a placeholder force, where the check shows that a placeholder stands where no force of the record stood. Each change is set down in this record under "After the loading check" with its reason.
3. **What may not:** a pass mark; a condition; a figure of a doctrine or of the rules; and anything at all after a judged run has been made.
4. **What the check must say about each condition:** whether the setup lets the condition be met at all. Where it cannot, for a reason of data and not of the staff, that goes to the architect before the run; where the architect leaves it, the condition is run and its failure is named with that cause.

One such case is already seen and is ruled here. The setup's surface forces of both sides are placeholders that lie in the Coral Sea, 14 to 18 hexes from the airfield (decision 0128, reading 3), on the surface combat mission, and no force of either side is on the bombardment mission. No staff can bombard with them or reach the airfield's waters in a night. The record has Japanese ships shelling the airfield and entering its waters by night from Shortland through these weeks. **The architect's ruling:** before the loading check the Japanese placeholder night force A is restated in the staff's setup alone as a placeholder bombardment force lying stopped at Shortland with Shortland as its home port, from the day the setup already gives it; the scripted setup is not touched. It stays a placeholder and is named so. Whether Shortland holds the fuel it needs is read in the loading check.

## How the conditions are read

Each is read from the umpire's export of each seed and from the batch file, by scripts written and tested on the scripted setup before the run, as decision 0128's were.

1. **It runs.** Every seed's game runs to its end with no invariant broken; each arrangement is run twice and the batch files are the same, and the exports of a sample of seeds are the same.
2. **Refused.** An `order_rejected` for an order a staff gave, and any `command_refused` for a command a staff gave, on any seed. A commander's command of the table above that is refused on some seed is counted apart and reported.
3. **The Allied side does something,** each counted in seeds:
   - **lands men on Guadalcanal:** a `men_landed` of an Allied unit in the hex of Lunga;
   - **takes the airfield:** a `base_changed_side` of base 1 to Allied;
   - **lands supply there after the landing:** supply in a `cargo_unloaded` at an Allied base in the hex of Lunga on a day after the first Allied man landed there, from a force other than the one that landed him;
   - **flies from the field:** a search, a patrol or a strike by an Allied group based at base 1, read as decision 0128 read them.
4. **The Japanese side does something,** each counted in seeds:
   - **lands men on Guadalcanal after the Allied landing:** a `men_landed` of a Japanese unit in the hex of Lunga on a day after the first Allied man landed there;
   - **attacks the airfield ashore:** a `ground_fight` in the hex of Lunga in which the Japanese side attacks, after base 1 is Allied;
   - **bombards it or enters its waters by night:** a `shore_bombardment` of base 1 by a Japanese force, or a Japanese force on the bombardment or the surface combat mission that ends a night pulse in the hex of Lunga.
5. **Better than a side given nothing.** The median over the seeds of the Allied margin on the objectives scale in (a) is above its median in (b); the median of the Japanese margin, the negative of the Allied, in (a) is above its median in (c).
6. **Legibility.** Below.
7. **Speed.** `sim-cli time` on arrangement (a), the median of a staffed day over the 39, on the machine and with the workloads decision 0100 names: no more than 40 ms.
8. **The operation on placeholder forces** is judged by the hand calculations of scenarios 66 to 77, which pass or do not.

## Legibility

Twenty-four reasons, taken by rule and not by eye, from seed 1 of arrangement (a): four from each of six doctrines, the first four reasons of that doctrine that rest on an order or a staff command, in event order, both sides together.

| Doctrine | Its reasons |
| --- | --- |
| Units made ready | An order of "ready a landing" or "build up a base" (decision 0141) |
| The force chosen | The forming of "ready a landing" under a take, and a later lift (decisions 0142, 0149 and 0152) |
| Ashore | A posture order under a take or a hold (decisions 0143 and 0144) |
| The supply that follows | A run under a hold's "keep supplied", and a force with men aboard sent by a hold (decisions 0144 and 0152) |
| By night | A run by night's speed or route, and a bombarding force's route (decisions 0145 and 0146) |
| Cover and reconnaissance | The forming of a carrier task under a take, a base strike order, and a reconnaissance order (decisions 0147 and 0148) |

Where seed 1 gives a doctrine fewer than four, the rest are taken from seed 2, then 3. A doctrine that gives fewer than four on the first ten seeds is reported as giving none to judge, and the test is of the reasons there are. The same cheap model answers the same two questions on each, as in decision 0128: what the staff is trying to do, and what would change its mind. **It passes at 20 of 24,** as decision 0139 has it, and no doctrine's reasons all fail.

## Reported and not judged

As decision 0139 has it, and with these:

- **Who holds the airfield on day 39,** each side's men lost and ships sunk and out of action, and the margin's spread.
- **The same run with command levels on** and the live report figures, on 100 seeds.
- **The staff's orders beside the script's,** day by day, on seed 1: the days each force of the record sailed against the days the staff sent it.
- **A model's game** of the 39 days by intent alone, once, with its account.
- **How often each limit above showed:** forces sent home for fuel, runs that went by day because no speed reached the dark, bombardments stopped on rounds, lifts that waited for want of a count.

## What is done with a failure

A condition that fails is reported with its seeds and its cause from the staff's own reasons. Nothing is tuned to pass it. What it shows is put to the designer with the choice it raises: a doctrine to change, a limit of the stage to lift, or a miss to name.

## After the loading check

Nothing yet.

## As run

Nothing yet.
