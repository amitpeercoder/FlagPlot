# 0109: The first tuning round of the ground figures: what is settled first, and the plan

Status: a plan by the architect, written on Nov 14, 2026, from four runs of the lengthened slice (`docs/validation/full-slice-39-days-baseline.md`, `-after-0105.md`, `-after-0106.md`, `-after-stock-fix.md`) and a model's five days in the Allied seat. It follows the rules of tuning in `docs/step2-spec.md`. Decision 1 corrects a rule and needs the designer's confirmation; decisions 2 to 4 settle data and are the architect's; the round itself is run after them. Nothing is built and no figure has moved.

Summary: The first tuning round of the ground figures: before it, men who are falling back are not fired on unless the victor attacks, the Japanese runs land their own rations as cargo, and riflemen set to work make a tenth of a labourer's work; then one round on the Tenaru and the ridge, mechanism by mechanism, with the figures each may move and the direction a source supports

Standing: Plan. Decision 1 needs review. Not built

## Where the slice stands

After the builds of decisions 0104 to 0106 and the stock fix, over 1,000 runs:

| Measure | The record | The slice | Verdict |
| --- | --- | --- | --- |
| Lunga and Tulagi American, by day and at the end | yes | every run | agree |
| American men lost at the Tenaru | 109 to 110 | median 23 | miss |
| Japanese men lost at the Tenaru | 800 to 900 | median 340 | miss |
| American men lost at the ridge | 134 to 253 | median 101 | reported |
| Japanese men lost at the ridge | 600 to 1,138 | median 1,900 | reported |
| Days of food at Lunga when the transports sail | 17 to 37 | 68 | reported |
| The day the airfield is built | 17 to 19 August | 17 August | reported |

Four things in those runs are not figures to tune. They are settled before the round, because a figure fitted over a fault hides the fault (the spec's rule 6: what the figures rest on is checked first; and rule 8: a scenario's orders and data are set before the first round and then left alone).

## Settled before the round

### 1. Men who are falling back are not fired on, unless the victor attacks

Decision 0104, 6, says a victor who goes on attacking fires on men who have not yet left. It follows that a victor who does not attack does not. The code does otherwise in one case. At the ridge the Kawaguchi parts are beaten off on 12 September and are marching out. On 14 September the Oka group attacks in the same hex, the Marines defend, and the losses of that fight are shared among every Japanese unit in the hex in proportion (decision 0090, 9): the men leaving lose 709 in the median run, to a fight they are not in. They also count in the limit of decision 0105, so the Marines fire as if 2,200 men were attacking and not 236.

**The correction:** a unit that is falling back, by decision 0104 or 0105, takes no share of a fight's losses and is not counted among its side's fighting men for the limit on engaged men, on a day when the other side is not on attack in the hex. On a day the other side attacks, it is fired on and counted as now. A unit marching by order is as it was.

This is a rule's wording caught up with its intent, and it changes results, so it has its own hand calculation and the designer's confirmation.

### 2. The Japanese runs land their own rations

Decision 0101 put a stock ashore in each landing hex before anyone landed, ten days and then seven for the men due there. The stock fix showed what that does: men who arrive early eat what was meant for men who arrive late, the stock is gone by 26 August, and the Oka group reaches the field in September with a fed share of 0 and fires nothing.

A row says how it was: two weeks' rations were carried with the men by the destroyers (LUF 975), and a destroyer was planned to carry 110 men and two weeks' rations (TL 170, 177, 178). So each run lands rations with its men, as cargo, and the stock stated ashore is cut to what the men who fall back there need:

- Each run of the Kawaguchi main body, the Aoba part and the Oka group carries 14 days for its men at the game's charge. The Ichiki echelon, the Yokosuka force and the force of 29 August carry 7 days, by the practice of row LUF 1044, which is all that bears on them.
- The carriers are the placeholder fast transport with cargo, as decision 0101's ruling D made for the American run of 15 August. A run's ships are as many as its men need.
- The stock stated in (2,-5) is 71 t, the seven days of amendment 8 for the garrison and construction units who fall back there. The stock stated in (3,-4) is 0: everyone who lands there brings his own.
- Each is marked a placeholder resting on a stated practice, as before.

### 3. Riflemen set to work make a tenth of a labourer's work

`fighting_unit_work_share` is 0.5. A model playing the Allied side put two combat groups of 4,300 men each to the airfield: each made about 2,150 of work a day against the pioneers' 660, and the field was finished on 11 August. The record has it finished on 17 to 19 August by the engineers and pioneers, with captured Japanese machines (PF 56, 198), and no row has riflemen building it. The figure becomes `"0.1"`: a regiment digs as well as a few hundred labourers with tools, not as well as two thousand. It is a placeholder moved toward the one thing the record supports, that the field was the engineers' work. The hand calculations run on the frozen figure and stand.

### 4. The Oka group's day

Decision 0101 has the Oka group march on day 37 and attack on day 39, 14 September, the day two rows give (LUF 1007, 949). That stands: it is set from rows. With correction 1 its attack no longer costs the Kawaguchi men anything, and with decision 2 it arrives fed.

## The round

It is run by a team, on the slice, after decisions 1 to 4 are built and a new baseline is taken. One mechanism at a time, in the order the rules run (the spec's rule 2), each step recorded with the report before and after in `docs/validation/tuning/` (rule 3). One set of figures serves both fights and every placeholder scenario (rule 1). The hand-calculation tests stay on the frozen figures.

| Step | What is wrong | Figures that may move, and the direction | What supports the direction |
| --- | --- | --- | --- |
| 1. Who can fire | Ichiki's 916 men reach the field with 412 able to fire | `unprepared_landing_fatigue`, down; `fatigue_per_march_day`, down | The echelon landed unopposed at night and marched two days (LUF 1049, 994, 1020); the record has it attack in strength |
| 2. The limit | Fire counted for three times the attackers | `engagement_ratio`, either way | A reinforced battalion with artillery met about 900 men at the Tenaru; two battalions and the artillery met about 2,000 at the ridge (EO 1924, 2013) |
| 3. What a hit does | Both sides lose too few at the Tenaru: 23 and 340 against 110 and 800 to 900 | `men_per_hit`, up; then `hit_chance` and `defender_hit_chance` for the balance between the sides | The losses of both fights on both sides (the measures' rows) |
| 4. When an attack stops | The Tenaru was one night; the ridge was two nights | `failed_attack_fatigue`, `attack_fatigue_limit` | The hours of the attacks (LUF 1017 to 1024, 1001 to 1014) |
| 5. Cover | Shallow fieldworks save about half of a raid's men, not the share the figure states, because small shares are cut to whole men | Not a figure: the team reports what the cut does at each level of cover, and the architect rules whether the rule is changed | The rule's own table |
| 6. The landing | 68 days of food ashore when the transports sail, against 17 to 37 | `unprepared_landing_share`, down; `handling_t_per_man`, down, so that men still land in a day | Less than half the supply was landed in 26 hours of unloading, and the men were ashore on the first day (LUF 465, 773, 688) |

Rules of the round, beside the spec's:

- **The two fights are fitted together and nothing checks them.** Every figure the round moves is recorded as fitted on two fights and not checked, as the spec has for the surface figures. The Tenaru's two measures are judged; the ridge's are reported, since the game cannot say which part of each side was engaged.
- **A step stops when its measure's record is inside the middle half of the runs,** or when moving its figures further would take another measure out. A measure left at the edge is said to be, with the reason.
- **No figure moves for one scenario,** and none moves to make up for something another step owns. If the Tenaru and the ridge want different values of one figure, the round stops and says so: the rule is wrong, not the figure.
- **The placeholder scenarios are rerun at the end** on the live figures, and each whose result changes is listed in plain words.
- **The round ends with a report** in the shape of the battle reports of step 2: every measure before and after, each figure moved with its old and new value and what it rests on, and what the round could not reach.

## What the round will not mend

- **The landings meet no enemy at sea or in the air.** The slice has no covering force, no night action off Savo Island and no raid on the transports, so the transports' sailing is by order. That is data and rules for a later round.
- **No ship shells Lunga,** so strain and wear under bombardment are barely exercised.
- **Real units are all alike.** Experience and skills are 2 for every unit until the rule that reads the research is written.
- **What a side is told.** The model in the Allied seat could not tell that its own attack was stopping work on the airfield, that the squadrons were due later, or that the enemy was leaving. Those are for a record of their own on the briefing.

## Rule card, for correction 1

| | |
| --- | --- |
| Reads | Whether a unit is falling back; whether the other side has a unit on attack in the hex |
| Writes | The units that share a fight's losses; the fighting men counted for the limit on engaged men |

## Question for the designer

1. **Correction 1:** men who are falling back are fired on only when the side that beat them is on attack. I recommend it: it is what decision 0104 said, a victor chooses to pursue by ordering it, and without it a third party's attack destroys a retreat. The other choice is that men still in the hex are in every fight there, as now.
