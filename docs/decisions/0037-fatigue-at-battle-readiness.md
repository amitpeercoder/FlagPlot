# 0037: Fatigue at battle readiness

Status: confirmed. The designer chose on Nov 11, 2026 to give battle readiness a cost through the same fatigue number as air groups, so that a force held at battle readiness tires. Decisions 3 and 5 are choices that answer did not state and are marked for review. Decision 3 in particular: what tiredness does to a force.

## The gap

Battle readiness costs nothing (decision 0021). A player would order it for every force on the first day and never think of it again. The complexity review kept readiness on one condition: scenario 6 gives it a cost, or it is cut and every force is simply alert.

## Decisions

1. **A task force has a fatigue from 0 to 100,** as an air group has (decision 0036). It starts at 0 unless the scenario says otherwise.
2. **At the start of each pulse, a force that is alert gains a figure and a force that is not loses a rest figure,** not above 100 and not below 0. Alert is as decision 0021 has it at the start of a pulse: at battle readiness, or at normal readiness and alerted by a contact. Night counts as any other pulse: a crew at its stations is at them in the dark too.
3. **Fatigue cuts a force's chance to sight the enemy, and does nothing else** *(needs review)*. In each round of a fight the force's chance to sight is multiplied by (100 less its fatigue) over 100. It changes no gunnery, no speed and no alarm roll.
4. **The figures are in the fatigue file,** with the air groups'. A scenario without the file has no fatigue.
5. **The staff does not stand a force down** *(needs review)*. The player orders readiness, and the order is the decision. A rest threshold for forces, as air groups have, can come with the staff rules if the order proves to be clerical work.

## Why

- **One effect, and it is the one that failed at Savo Island.** The crews of the southern group had been at their stations for two days, and their lookouts did not see the Japanese column. A tired force here is one that sees late.
- **It makes the order a judgement.** With the placeholder figures a force at battle readiness sights better than a rested force caught unready until its fatigue passes 50, about three days. After that it sees worse than if it had rested, though it is still not surprised. So readiness is worth ordering when a fight is expected and not before.
- **A cut that grows, not a wall at 100.** A force forced to stand down at a level would cost nothing until that moment. A chance that falls each pulse shows in every sighting roll.
- **Why not fewer guns firing,** by analogy with fewer aircraft flying. Gunnery already has four factors. Sighting has two, and the sighting roll is where readiness already acts.

## What it costs

- A force that is always alerted by contacts tires as one at battle readiness does. A force shadowed for a week arrives tired.
- The night pulse is twelve hours and each daylight pulse six, and each costs the same.
- Savo Island's hand calculation is unchanged: it names no fatigue file. With the file and two days at battle readiness the southern group would sight less often. That is for step 2 to tune.

## Deferred

- A rest threshold for task forces.
- Fatigue from steaming at high speed and from fighting.
- An effect on the alarm roll or on gunnery.

## What the runs show

Over seeds 1 to 1000, the night fight of scenario 6c2. All figures are placeholders.

| | Force A sights in round 1 | Force B sights in round 1 | Ships sunk a run, A and B |
| --- | --- | --- | --- |
| Force B two days at battle readiness, force A since the evening | 455 | 319 | 0.363 and 0.346 |
| No fatigue file | 487 | 493 | 0.544 and 0.322 |
| Force B at normal readiness throughout | 455 | 248 | 0.395 and 0.353 |

With no fatigue the fresh-arrived attacker loses more ships than the defender. With it the two days at battle readiness cost force B the advantage, and standing at normal readiness would have cost it more.

## The legibility test

A cheap model was given the Allied briefing and the log up to the first round of the night fight. It worked both sighting chances from the log, 0.325 and 0.475. It worked the two cases it was asked about: 0.25 for the force kept at normal readiness and 0.475 for one ordered to battle readiness on the third day. It found that after ten pulses at battle readiness a force sights no better than a rested one at normal readiness, and stated the trade-off. It asked one thing the briefing did not say: whether a force sighted in a round is alerted before its own roll. Decision 0014 answers it; the briefing for a real game must too.

## Rule card

| | |
| --- | --- |
| Reads | A force's readiness and whether it is alerted at the start of the pulse; its fatigue; the fatigue figures |
| Writes | A force's fatigue; the chance of its sighting rolls |
| Events | Force fatigue gained or rested, when it changes |
| Randomness | None |
