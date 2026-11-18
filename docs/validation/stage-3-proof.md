# Step 3, stage 3: the proof of taking and holding a place

Run on Nov 18, 2026, by the architect, to the plan of decision 0153, written before the run, for the pass marks of decision 0139. Release build, live figures, seeds 1 to 1,000, from main at f92f8d8d (the merge of pull request 418): state layout `flagplot-state-v44`, rules label 19, doctrine `supply-34`, agent interface 24, saved game 19. Nothing was tuned, and no rule, figure, reading, pass mark, intent or file was changed after the run.

**Result: not passed as stated.** Two conditions fail, both the Japanese side's, and both for one cause. Every other judged condition meets its mark.

**Closed by the designer on Nov 18, 2026, with both as named misses** (the first of the three choices below; decision 0153, "Closed on Nov 18, 2026"). The report below is as it was written before he decided.

| Condition | Value | Mark | Holds |
| --- | --- | --- | --- |
| 1, it runs, and the same start gives the same game | 3,000 games run to day 39; each arrangement's two batch files byte for byte the same; five exports of each made twice, the same | every seed; identical | yes |
| 2, refused | 0 staff orders rejected, 0 staff commands refused, 0 commander's commands refused, on 1,000 seeds | none | yes |
| 3, the Allied side lands men on Guadalcanal | 1,000 seeds | 900 | yes |
| 3, takes the airfield | 1,000 | 800 | yes |
| 3, lands supply there after the landing | 1,000 | 800 | yes |
| 3, flies from the field | 1,000 (a patrol and a strike on 1,000, a search on 906) | 600 | yes |
| 4, the Japanese side lands men on Guadalcanal after the Allied landing | 1,000 | 800 | yes |
| 4, attacks the airfield ashore | **0** | 600 | **no** |
| 4, bombards it or enters its waters by night | 1,000 (a bombardment on 1,000) | 600 | yes |
| 5, the Allied side better than given nothing | median margin 64.65 from its intents, -59.95 given nothing | above | yes |
| 5, the Japanese side better than given nothing | median margin **-64.65** from its intents, -51.51 given nothing | above | **no** |
| 6, legibility | 23 of 23 reasons judged pass both questions | 20 (five in six of 23, rounded up) | yes |
| 7, speed | 17.06 ms a staffed day (Allied 7.13 ms and Japanese 9.93 ms, medians over the 39 days, Apple M3 Pro) | 40 ms | yes |
| 8, the operation on placeholder forces | the hand calculations of scenarios 66 to 74, 76 and 77 pass on main at f92f8d8d (CI success) | pass | yes |

## What was run

The three arrangements of `data/scenarios/stage-3-proof/` (`make_arrangements.py --check`: each file as made): (a) both sides from intents, (b) the Allied side given nothing, (c) the Japanese side given nothing. With `scripts/stage-3-proof/run.sh`, in order: the build; the loading check again on seed 1; each arrangement's batch of 1,000 seeds twice; the 1,000 exports of (a); the sample of seeds 1, 2, 3, 500 and 1,000 of each arrangement, exported twice; `sim-cli time` on (a); and the reading scripts. The outputs are kept outside the repository.

## The two conditions that fail

### The Japanese side never attacks the airfield ashore: 0 of 1,000

This was seen on the loading check and run as it stood (decision 0153, "After the loading check").

The Japanese staff lands every lift at Lunga, a median of 6,076 men a game, and each morning weighs the men it has there against its estimate of the enemy. At balanced, the risk a commander who sets none is given, it wants two men for each enemy man it counts (decision 0143), and it never has them:

- On the last morning it has a median of 4,673 fit men against 20,025 wanted, on an estimate of 10,012 enemy men. The estimate is a fair one: the Allied side has a median of 11,056 men at Lunga.
- At its best morning of a game it has a median of 0.37 of what it wants. On 31 seeds of 1,000 it ever has half; on none all. The most is 0.72 (seed 308, day 39: 5,505 fit men against 7,600 wanted).
- On seed 1, day 39 (event 10288): "the units that can attack have 4789 fit men against the 9728 wanted, short by 4939 ... The men that can attack are too few and none is on attack: the staff waits."

So the staff does what its doctrine says, on a count that is about right. The record's attacks on the airfield were made on an estimate of some 2,000 Americans, a fifth of the truth. The game's Japanese side knows better than the record's did, and a staff that knows it is outnumbered two to one does not attack at balanced.

### The Japanese side does worse from its intents than given nothing: -64.65 against -51.51

Read from the result lines of `sim-cli run` on every seed of (a) and (c), each checked against the batch file's margin and hash. On the mean the Japanese margin is 10.3 points worse from its intents. By objective, the mean change to the Japanese margin:

| What changes | Points |
| --- | --- |
| Japanese men lost | -15.8 |
| Japanese aircraft destroyed | -4.9 |
| Allied cargo landed at Lunga | -4.8 |
| Japanese ships sunk | -4.4 |
| Japanese ships out of action | -3.1 |
| Allied men lost | +12.8 |
| Allied aircraft destroyed | +2.9 |
| Allied ships sunk and out of action | +4.1 |
| The airfield closed more days (both sides' share) | +2.9 |

The largest loss is the same cause as the first failure. The men the lifts land wait at Lunga for an attack that is never ordered, and they starve. The lifts bring a median of 348 t of supply for 6,076 men over the whole game; the landed units lose a median of 1,543 men unfed and 45 to fire. Checked on seed 1: 6,076 men landed in units 16 to 25, of whom 1,491 are lost unfed; the garrison that began ashore loses 1,539, the same as when the side is given nothing. No rule of stage 3 feeds men ashore at a place that is not yet the side's: "keep supplied" belongs to a hold, and a hold needs a base.

The rest is the price of fighting at all: a carrier battle and raids the side does not fight when given nothing (aircraft, the Shokaku out of action on 503 seeds, lift ships sunk after landing their men), and the Allied supply convoy of 960 t, which sails on 998 seeds of (a) and never in (c) because the Japanese raids and bombardment destroy a median of 2,500 t at Lunga and the Allied staff answers. What the Japanese side gains is mostly the other half of that: Allied men unfed on 638 seeds, from about day 36, once Lunga's stock is spent.

The margin's spread, the Allied side's: (a) from -13.0 to 97.1, quartiles 50.6 and 74.2, 994 decisive and 5 major Allied victories and 1 major defeat (seed 65, where 5,980 Allied men starve); (b) -108.5 to -22.2, 1,000 decisive defeats; (c) 49.8 to 56.1, 1,000 decisive victories.

### What the two raise, for the designer

Both come from one thing: the Japanese staff carries men to a place it then judges it cannot take, and has no rule for what to do with them. Three choices, as decision 0153 has them:

1. **Name both as misses and close the stage.** The staff's judgement ashore is sound on what it knows; the record's attacks rested on an estimate the game does not reproduce. The architect's recommendation, with the two mends below taken up at the start of stage 4.
2. **Change a doctrine and run again.** Two candidates, each a decision record with its hand calculation first: men ashore at a place not yet taken are fed by runs, as a held base is; and a take whose count says the men it can bring will not be enough either does not lift them or lands them at a beach away from the enemy and waits there (the staff cannot choose a beach today, so every lift lands in the airfield's own hex, where the record's went to Taivu and the coast west of the Lunga).
3. **Restate the condition.** The Japanese commander of the proof set no risk. A commander who states "press" lowers what the staff wants; that is a different statement of intent, to be written down before a second run and reported beside this one.

## Legibility: passes, 23 of 23

`crates/sim-cli/tests/recorded_games/legibility_stage_3/` holds the reasons, the key, the answers and the marks.

The picking rule took 23 reasons from seed 1 of (a), not 24: "Units made ready" gave three on the first ten seeds, the rest being word for word the same as one already taken. By reading 6 of decision 0153 the test is then of the reasons there are and passes at five in six rounded up, 20. The key was written from the reasons alone before any reader ran. Twenty-three Haiku readers, each given one reason's five parts and the two questions, answered; a Sonnet model marked them against the key and the architect read the marks. All 23 pass both questions, in every doctrine.

What it did not reach: no later lift, bombarding force's route or reconnaissance order was among the first four reasons of its doctrine, and all but two of the reasons are the Allied side's. The row for a force with men aboard sent by a hold, whose words had not been met before the run, was met (reasons 12 and 13).

## Speed: passes, 17.06 ms

`sim-cli time` on (a), Apple M3 Pro, release: a day's staff decisions take a median of 7.13 ms for the Allied side and 9.93 ms for the Japanese over the 39 days, 17.06 ms together against 40 ms. The longest single day is 42.5 ms (Allied, day 1) and 37.1 ms (Japanese, day 31). A day of the slice resolves in 0.6 ms early and 11.3 ms on day 13.

## Reported and not judged

**Who holds the airfield on day 39.** In (a) the Allied side holds Lunga and Tulagi on all 1,000 seeds, both taken on day 1. In (b) both stay Japanese; in (c) both are Allied.

**Losses in (a),** median (least to most): Allied men 910 (141 to 6,258), Japanese men 3,672 (2,537 to 4,425); Allied ships sunk 1 (0 to 4), Japanese 2 (0 to 7); out of action, Allied 0 (0 to 2), Japanese 1 (0 to 3).

**How often each limit showed, in (a):**

- Forces sent home for fuel: none. No carrier force has a home port in the setup, so its fuel is never weighed; the limit could not show.
- Runs that went by day because no speed reached the dark: the Allied first landing at Lunga on day 1, on every seed, and nothing else. No Japanese lift goes by day.
- Bombardment stopped for want of rounds: on 816 seeds. The force fires on the night of day 18 on every seed and on a second night on 105; Shortland holds no ammunition in the setup.
- Lifts that waited for want of a count: none.
- "Cannot meet" notices: a median of 91 a game from the Allied staff and 176 from the Japanese. Most are raids and neutralisings beyond the bombers' normal reach (Shortland and Espiritu Santo, and Rabaul's raids on Lunga and Tulagi).

**The staff's orders beside the script's, seed 1.** Every Japanese lift sails on the day the script sails it, and goes to Lunga itself where the script sends it to Taivu or the coast west of the Lunga. The Allied transports and the fast transports of 15 August and 1 September sail on the script's days. The Allied supply convoy sails on day 24 against the script's day 16, and the cargo ships of 21 August (task force 23) are never sent: they are not in the forces limit the hold set on day 1. The Allied carriers move from day 18 where the script gives them no route. The Tulagi crossings and the placeholder night forces other than the bombardment force are not sent.

**Command levels on, 100 seeds,** with the live report figures, a copy of (a) made outside the repository: every count of conditions 3 and 4 is as on (a)'s first 100 seeds (the Japanese side never attacks ashore), nothing is refused, and the median Allied margin is 61.45 against 62.03.

**A model's game** of the 39 days by intent alone has not been played. It is owed.

## Known before the run and still so

The measure of an island keeps no record of past days (decision 0152); the readiness order after a neutralising is withdrawn is not given; a carrier weighs fuel and not rounds; a second lift under a limit the staff assembled finds no unit. None bore on a judged condition.
