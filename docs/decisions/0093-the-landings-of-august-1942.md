# 0093: The landings of August 1942 and the first attacks on the airfield, as a scenario

Status: a plan by the architect, written on Nov 14, 2026. It is stage 4 of the ground combat plan the designer approved (`docs/design/ground-combat.md`). Decisions 2, 4 and 6 are choices that shape what the scenario is and need review. Nothing is built. The data choices are the architect's, each with its reason, and none is made final here: the rows are read when the scenario is written.

## What stage 4 is for

Stages 1 to 3 run on made-up units. Stage 4 puts the record through them: the landings of 7 to 9 August 1942 on Guadalcanal and Tulagi, the Japanese landings that followed, and the two attacks on the airfield inside the period the facts cover well, the Tenaru on 21 August and the ridge on 12 to 14 September. It is where the ground figures first meet something a source says.

## What the facts will bear

From `docs/research/oob/land-forces.md` and the rows behind it, all still unconfirmed:

- **American men ashore** are well covered: 10,000 to 11,000 on Guadalcanal and 6,075 on the Tulagi side by 9 August.
- **The Japanese garrison** is disputed between the two Japanese-derived accounts: 240 or 250 naval guard troops and 1,600 or 2,700 construction men on Guadalcanal; about 440 on Tulagi and Gavutu by one account and 780 by American counts.
- **The Japanese forces sent** are known as totals: about 900 or 1,000 in the first echelon of the Ichiki Detachment on 18 August; about 5,200 or more than 6,000 of the Kawaguchi Detachment between 30 August and 7 September, of whom about 2,000 attacked at the ridge.
- **Losses in each attack** are rows on both sides, and every row for "the Japanese attack held ground" says it did not.
- **What each side believed** has rows: the Japanese took the landing for 1,000 or 2,000 men.
- **Troop capacity** is stated for five American transports, and loads on one voyage for seventeen more ships. No destroyer of either navy has a stated capacity; there are run totals of about 150 men a ship.
- **Nothing sizes** how fast an unfed force wasted, what a Japanese soldier ate, or how fast men were put ashore.

## Decisions

### The scenario

1. **One new scenario, 39 days long,** from the night of 6 August to 14 September 1942, on the real map. Day 1 is 7 August. It is a scenario of its own first; land units go into the full slice and the soak after it passes (decision 10).
2. **It holds the men, the bases and the ships that carried men, and nothing else.** No carrier, no cruiser force, no air group. The sea and air battles of the same weeks have their own scenarios, and the full slice is where they meet the land fight. *Needs review: the other choice is to add the land units straight to the full slice. That puts two untested things together, and the full slice is 14 days, too short for the ridge.*
3. **No force enters late.** The format has no arrival of ships by day. Every force is in the scenario from day 1, at the base it sailed from, and is ordered out on the day the record gives.

### Where men are

4. **Every landing on Guadalcanal's north coast is put in the Lunga hex.** The hexes are 40 miles across and men do not move overland. Taivu Point, where the Ichiki and Kawaguchi men landed, is about 22 miles east of the Lunga and falls in the hex next door; the Japanese then marched. With no march in the rules, a force that landed to attack the airfield is landed in the airfield's hex, where it forms its own base, hidden until a ground report finds it. *Needs review: this makes the march free and instant. The other choice is to wait for movement overland, which the plan defers to a later rule of its own.*
5. **Tulagi, Gavutu and Tanambogo are one place,** the Tulagi hex.

### Who is a land unit

6. **A unit's men are its fighting men.** Construction workers and labourers are not counted: the Japanese construction units on Guadalcanal, 1,600 to 2,700 men by source, did not fight and left the airfield on the first day. The garrison on Guadalcanal is the naval guard troops. *Needs review: counted as soldiers, the labourers would make the airfield cost several days and several hundred Marines to take, which no source supports. Left out, they also do not eat. The record's dispute over their number then does not matter to the game.*
7. **A unit is what the record names as landing together:** the Guadalcanal force and the Tulagi force of 7 August, the first echelon of the Ichiki Detachment, the Yokosuka 5th Special Naval Landing Force, the Kawaguchi Detachment's echelons as far as the rows give them. Where sources disagree on men, the measures file takes the range and the scenario takes the row the rules file names, with the reason.

### Ships that carry men

8. **The American transports and the destroyer-transports become real classes,** each with a troop capacity derived by a written rule: the stated capacity where the ship dictionary gives one, else the men she carried on 2 August. The ships and classes are added to `data/oob` and `oob-derive` gains the figure. A Japanese destroyer's troop capacity is derived from the run totals, by a rule that says it rests on runs and not on a stated figure.

### Supply

9. **Stocks are set from the rows on days of rations,** turned into tons by the game's own charge for a man, so that the Americans begin with the days of food the record gives them and not a tonnage the record does not give. The Japanese rows have no figure inside the period: their stocks are placeholders, marked.

### After it passes

10. **Land units are added to the full slice and the soak** in a second pull request, with the soak's invariants extended to men: conserved, never negative, never more aboard than a ship's capacity.

## The measures

A few and coarse, as the plan's second answer said. They go in `data/validation/` with the rows they rest on, and `sim-cli validate` gains what it needs to read them:

| Measure | The record | Judged |
| --- | --- | --- |
| The airfield is American by the end of day 2 | Taken on 8 August | Yes |
| Tulagi is American by the end of day 3 | Secured by 9 August | Yes |
| The airfield is American at the end of 21 August | Every row: the attack held no ground | Yes |
| The airfield is American at the end of 14 September | The same | Yes |
| American men lost at the Tenaru | 34 or 35 killed and 75 wounded | Yes, as a range |
| Japanese men lost at the Tenaru | About 800 of 900 to 1,000 | Yes, as a range |
| American and Japanese men lost at the ridge | 31 to 59 killed, 103 to 194 wounded; 600 to 1,138 | Reported, not judged |

The ridge's losses are reported only, because the game fights the whole of both sides in a hex and at the ridge a small part of each was engaged. A man "lost" in the game is out of the fight, so the record's killed and wounded are added.

The ground figures are then tuned under the rules of step 2: one set for every battle, a figure moved only toward what a source supports, the hand-calculation tests kept on the frozen figures. Two battles are few. What is fitted on them is recorded as fitted and not checked.

## What it will show that is wrong

Known before a run, and recorded so that nobody is surprised:

- **A fight lasts until fatigue stops it,** four days on the placeholder figures. The Tenaru was one night.
- **Every man in the hex fights.** The game sets 900 men against the 10,000 on the island, not against the battalion on the river.
- **The march is free** (decision 4), and so is the Japanese approach by sea: no American ship or aircraft is in the scenario to stop a destroyer run.
- **Sickness is not modelled,** and it took more Marines out of the line than the fighting did.

## What it costs

- **Data:** new ships and classes in `data/oob`, a new figure and two rules in `oob-derive`, a new derivation list with its pinned class ids, and the generated class file.
- **The validation harness:** takes for a base's holder on a day and for men lost by side between two days, and the batch record's counts behind them.
- **A tuning round,** with its record under `docs/validation/tuning/`.
- **No new rule.** If the scenario cannot be built without one, the work stops and the rule gets its own record.

## Questions for the designer

1. **Decision 2:** the scenario holds only the men, the bases and the ships that carried them. Agreed?
2. **Decision 4:** every landing on Guadalcanal's north coast is put in the airfield's hex, since men do not march. Agreed?
3. **Decision 6:** only fighting men are counted; construction workers are left out. Agreed?
