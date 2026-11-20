# 0194: The last day ashore

Status: a plan by the architect, written on Nov 20, 2026. It mends a fault of decision 0193 that four games between agents showed. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: With nothing more coming, a staff sends its men ashore in to attack. On the last day of a game that attack cannot take the place and can only lose men. A staff does not know which day is the last: the day count is on the briefing and not in what a staff reads. This gives a side's view the number of days the game has, and has the staff not send men in under decision 0193 when too few days are left for it to matter.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

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

## Amended on review

A reviewer read the plan against the fight ashore's figures, the trial and the code. Found right: that the staff's morning is well defined against the days resolved; that the judgement sits in one place; that the game's length is configuration common to both sides.

7. **The reason given for two days was false.** At the live figures an attack at even numbers against men dug in fails on its first day, tires the attackers past their limit on the second, and would need about eight days to beat the defenders; at twice their number about five. So "an attack cannot take the place with one day left" is true of two days as well, and of most attacks. The rule is restated as what it is, the staff's judgement, and its words say so: **with fewer than 3 days left the staff begins no attack ashore, because an attack begun so late only costs men.** The 3 is the staff's figure, a placeholder.
8. **It covers every attack the judgement would begin,** not only those of decision 0193: with fewer than 3 days left no unit on defend is set to attack, whether the men are enough at the risk's number, or nothing more is coming, or no estimate is held. A wait for this reason hands in no notice.
9. **Men already in, on the last days.** Rule 4 is replaced. With fewer than 3 days left, a unit on attack whose side's attack in its hex failed the day before is set to defend, and the reason says so. A unit on attack whose attack did not fail stays in.
10. **"More is coming" ends with the game.** In decision 0193's test, a unit whose day is after the game's last day, and a lift that cannot arrive by it, is not coming. Today such a unit counts where the end has no date, and the staff waits for men who never come inside the game.
11. **The trial.** A world built from a picture carries the game's days, so that the staffs a trial plays judge as the live ones will. A trial may play past the game's end (chapter 9): the days left are then nothing, never less. Chapter 9 says so.
12. **Not state, and no hash moves.** The game's days are set once where the scenario makes the world, beside the objectives and the reference data, which are configuration and are not in the hash. No daily hash of any scenario moves; an old saved game replays as it did. The session reads the same figure, so the game's length has one source. The side's view and a seat's view give it; the horizon of the objectives is another thing (the Coral Sea has 5 days and a horizon of 30) and the rulebook says why.
13. **The worked example is restated by the architect:** scenario 109's first morning with nothing coming is day 4, so the days left are set by the game's length: 6 days for 3 left, 5 for 2.
14. **Games that move:** the Coral Sea with a person in the Allied seat (its men were sent in on the last day); the stage 3 proof's first file in its last days, if rule 10 changes what it waits for; whatever else the build's run of every scenario finds. The doctrine version and the rules label rise.
15. **Built first of the four,** before decisions 0195, 0196 and 0197.
