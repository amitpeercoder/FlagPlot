# 0194: The last day ashore

Status: a plan by the architect, written on Nov 20, 2026. It mends a fault of decision 0193 that four games between agents showed. To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: With nothing more coming, a staff sends its men ashore in to attack. On the last day of a game that attack cannot take the place and can only lose men. A staff does not know which day is the last: the day count is on the briefing and not in what a staff reads. This gives a side's view the number of days the game has, and has the staff not send men in under decision 0193 when too few days are left for it to matter.

Standing: Planned on Nov 20, 2026; waits for its review.

## Why

Scenario 105's games between agents (`docs/validation/coral-sea-agent-games.md`). Both Japanese commanders who landed overruled their staff on the morning of day 5: it had ordered the men in under decision 0193, and each kept them dug in, "because a last-day attack would only lose men the enemy scores for".

What is so today:

- **A side is told the game's length:** its briefing says "Day 2 of 5" and "This is the last day". The session holds the number; the side's view and the world do not, and a staff reads the view.
- **Decision 0193** sends men in when nothing more is coming and their fit men are at least the estimate (balanced), or whatever the estimate (press).
- **A fight ashore** is resolved day by day; a place is taken when its defenders are beaten, which a day's fight at even numbers against men dug in does not do.

## The rules

1. **A side knows how many days the game has.** The side's view gains the number of days of the game, the same for both sides and for a seat. It is configuration, as the levels of victory are, and says nothing of the enemy.
2. **The days left** on a morning are the game's days less the days resolved: on the morning of the last day, 1.
3. **A staff does not send men in under decision 0193 with fewer than 2 days left.** On such a morning the judgement ashore is today's for a wait: the units stay on defend, the reason says that nothing more is coming, that the men would be enough, and that with one day left an attack cannot take the place. No notice is handed in for it.
4. **Men already in stay in** (decision 0193, rule 12), on the last day too: the staff does not pull an attack out because the game is ending.
5. **Nothing else of the staff reads the days left.** A lift still sails, a strike still flies. Other things a commander would not begin on a last day are not looked for here.
6. **The figure, 2 days, is the staff's and a placeholder.**

## Rule card

- **Reads:** the game's days and the days resolved, from the side's view; what decision 0193 reads.
- **Writes:** nothing but the staff's reason.
- **New:** one reading on a side's view; one test in the judgement ashore. The game's days are added to the state the world holds so that the view can give them, and to the hash.

## What it costs

- **State and hash:** the world holds the game's days; the hash version rises and the golden files are remade.
- **Games that move:** those in which decision 0193 sent men in on the last day: the Coral Sea with a person in the Allied seat, and whatever else the build's run of every scenario finds.
- **Rulebook:** chapter 1 (what a side knows) and chapter 8 (Ashore).
