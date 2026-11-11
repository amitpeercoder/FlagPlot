# 0030: Escorts have a radius

Status: confirmed. The designer left the choice to the architect on Nov 11, 2026 ("figure this out and decide accordingly").

## The gap

Decision 0027 sends a fighter group's escorts with its carrier's raid "as far as it goes", and left a fighter's own range for stage 5e. Stage 5e has given fighters real figures: the F4F-4's sourced combat radius is 105 miles without drop tanks, and the A6M2's is 261. In the Eastern Solomons scenario the American strike flies 200 miles and the Japanese strike 240.

## Decisions

1. **An escort goes with a raid only if the raid's target hex is within the fighter type's radius** of where the fighter group is. The radius is the type's strike radius, measured as a strike's is: hexes times 40 miles.
2. **Fighters that cannot go stay on patrol.** If the raid their escorts would join is beyond their radius, the whole group that can fly is its patrol for that pulse.
3. **The test is made raid by raid, on the hex the raid is sent to,** which is where its side believes the enemy is. A group whose escorts go with a raid to an empty hex has still sent them.
4. **Which raid the escorts would join is chosen first,** by decision 0027, and the radius is tested against that raid only. They do not look for a nearer one.

## Why

Three things were weighed.

- **History.** Range was the Zero's clearest advantage in 1942, and it decided who had fighters over the target. On 24 August Saratoga's strike on Ryujo flew without fighters, and the Japanese first wave had Zeros with it. With the rule both come out of the figures. Without it, Wildcats escort at nearly twice their sourced radius.
- **Play.** A radius gives the player something to decide: close the range and strike with fighters, or strike from further off without them. It also gives the carrier's defence a reason to be strong when its strike is weak, because the fighters that could not go are overhead. Without the rule, distance costs a strike nothing but its bombers' own reach.
- **Simplicity.** It is the test a strike already makes, applied to one more group. It adds no figure: the type already has a radius.

Staying on patrol, and not simply staying on deck, is the choice that needs a reason. A commander who cannot send his fighters does not leave them idle while a raid may be coming. It also keeps "more fighters never make the defence worse" true.

## What it costs

- **The Wildcat's 105 miles is a cautious figure.** It is the Navy data sheet's combat radius without drop tanks, worked by the Navy's own method with its allowances for combat and reserve. The radius of the bombers that have no such sheet is a third of their range, which is a more generous method. So the rule may keep Wildcats at home on strikes they could have flown. With one drop tank the sheet gives 245 miles. Whether tanks were carried on 24 August is not known; if a source says they were, the figure changes and the rule does not.
- **A hex is 40 miles.** A fighter with a radius of 105 miles escorts to two hexes, 80 miles, and no further.

## Deferred

- Drop tanks as something a fighter group can be ordered to carry.
- Escorts that turn back part of the way.
- One method for every type's radius. Decision 0029 uses a sourced radius where there is one and a third of range where there is not, and the two are not the same kind of figure.

## Rule card

| | |
| --- | --- |
| Reads | The fighter group's position and its type's strike radius; the target hex of the raid its escorts would join |
| Writes | Nothing new: how many of the group are escorts and how many patrol (decision 0027) |
| Events | None new. `EscortsSent` is recorded only when escorts go |
| Randomness | None |
