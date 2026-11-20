# 0194: The last day ashore

Status: a plan by the architect, written on Nov 20, 2026. It mends a fault of decision 0193 that four games between agents showed. Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: With nothing more coming, a staff sends its men ashore in to attack. On the last day of a game that attack cannot take the place and can only lose men. A staff does not know which day is the last: the day count is on the briefing and not in what a staff reads. This gives a side's view the number of days the game has, and has the staff not send men in under decision 0193 when too few days are left for it to matter.

Standing: Planned on Nov 20, 2026 and amended on review; built on Nov 20, 2026 ("As built", below), its readings with the architect.

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

## As built

Built on Nov 20, 2026 on the branch `last-days-ashore`, to rules 7 to 15 as amended on review, with rules 1, 2 and 5 where they stand.

**What was built.**

- `crates/sim-core/src/world.rs`: the game's days, `game_days`, held as configuration beside the objectives (`World::set_game_days`, `World::game_days`); not state, named in `hash::name_every_field` as not hashed. No daily hash of any game moved for holding it. The rules label 25.
- `crates/sim-core/src/view.rs`: `SideView::game_days`, the same for both sides, for a seat's view and for a view held by the area command.
- `crates/sim-core/src/world_picture.rs` and `world_picture/half.rs`: a picture and a side's own half carry the game's days; `own_half` fills it from the view; `from_halves` joins it and refuses halves of games of different lengths; a world built from a picture is given it. `crates/sim-trial/src/enemy.rs`: the enemy's half made by rule takes the side's.
- `crates/sim-data/src/scenario.rs`: the loader sets the game's days from the scenario's `days`, where the world is made, and a scenario rebuilt on a new seed keeps them.
- `crates/sim-agent/src/session.rs`: `Session::new` hands its days to the world, and `Session::days` reads the world's: one figure for the briefing, the end of the game and both views.
- `crates/sim-staff/src/take/coming.rs`: `LAST_DAYS` (3, a placeholder), `days_left`, `in_last_days`, `last_days_words`, `attack_failed` (the side's own `GroundAttackFailed` at the place on a day); in `more_coming` the latest day is the earlier of the end's date and the game's last day. `take.rs`, the judgement ashore: in the last days no unit on defend is set to attack, a unit on attack whose attack failed the day before is set to defend, and the reason gives the days left. `lib.rs`: the doctrine version `supply-47`.
- Tests: `tests/scenario_110_the_last_day_ashore.rs` under the architect's statements, registered in `tests/Cargo.toml`; four unit tests in `coming.rs`. Scenarios 91 and 92's tests gained the new field of a picture and a half (`game_days`: none for a picture made by hand, scenario 1's 3 for a half).
- The rulebook: chapter 1 (What a side may know; A world built from a picture, item 1; A side's own half, items 1 and 7), chapter 8 (Taking a place, Ashore, items 7 and 11; the versions), chapter 9 (Playing a trial, rule 4), the glossary ("game's days", "days left", "last days", "more is coming"), `docs/rules/differences.md` rows 53 to 55 and a note on row 50, `docs/open-work.md`, the recorded games' saves (the rules label and the doctrine version; no transcript changed) and their README.

**Readings made in the build** (the architect's to confirm):

1. **The session's days.** `Session::new` keeps its signature and gives the world the days it is handed; the world then holds the one figure. Where a caller gives other days than the scenario's (some tests do), the world takes the session's (row 54).
2. **"A lift that cannot arrive by it"** is read as decision 0193 reads its date: a later lift as it stands is counted where it may first be used by the latest day, now the earlier of the end's date and the game's last day. Passage is not reckoned, and a lift of the take with men aboard counts whatever its route (row 53).
3. **The words of what is coming** keep the end's date, or "with no date", as before, so that on a morning with 3 days left or more the reason is decision 0193's unless the game's end leaves something out; a unit so left out is named "may be used from day 8, after the game's last day, day 6".
4. **"Whose attack did not fail stays in"** (rule 9) adds only the pull-out after a failed day: a unit on attack is not taken out for the last days, but where the judgement's own wait sets units on attack to defend (men too few while more is coming, or preserve the force), it still does (row 55). After a failed day the units on attack are set to defend whether the judgement would attack or keep them in.
5. **The notice.** A wait in the last days that the judgement would not have made hands in no notice; a wait the judgement makes for its own reason with nothing more coming still hands in `cannot_bring_enough` in the last days.
6. **"The day before"** is the last day resolved: on the morning of day D, a `GroundAttackFailed` of the side's at the place on day D - 1, as the side is told it.
7. **Rules 5 and 8 read together.** Only the judgement ashore reads the days left. A unit set to attack as it goes aboard a first lift is set so whatever the days left; on its first morning ashore it is judged, and stays in unless its attack failed.
8. **Not state.** The game's days are not hashed and are not in a saved game; a saved game replays from its scenario, which gives them. The agent interface is unchanged: the briefing already gave the day count, and no name or field a seat reads was added, so its version stays 35. The state layout and the saved game's version stay.

**The tests.** Scenario 110: every statement and property has a test; 19 pass and 3 are ignored, each with what the code gave:

- **Statement 1, the hashes of days 5 and 6.** With N = 6 the hashes of days 1 to 4 are those before this rule (pinned from the build before it), and the game's length moves no hash (N = 6 and N = 100 agree before the last days). Days 5 and 6 differ: on day 5, 2 days left, the attack of day 4 had failed and the staff sets U and V to defend. Before: 3205cf248fd6bcc8 and 1fa22d6e3a2b6d81; now 6d571fcf17840df1 and 08abe3394d0fbab8. The statement holds where the events do not move.
- **Statement 5's variant** (the defenders cut to 100 men) cannot be built as written: U and V go in on day 4 and beat the 100 that day, the base becomes the side's and the plan is complete on day 5, so there is no attack of day 5. Day 4 records `GroundAttackFailed` (a weight of 0 against 0.6: the men ashore are unfed and fire nothing) though the place falls that day; that is chapter 7's rule, not this one.
- **Statement 8** cannot be tested as written: the judgement ashore runs only for a released plan, and with U and V ashore at P from the start the staff forms no plan ("neither all ashore at the source nor all aboard"), so there is no judgement on day 1. No posture is ordered.

In the base game of scenario 109 (7 days) the Japanese men are unfed from day 5 and fire nothing, so every attack after day 4 fails.

**What it moved.** Every scenario under `data/scenarios` (the step 2 frozen copies left out) was run with `run --events` and `hash` on the build before this rule and on this one, and compared:

- `data/scenarios/coral-sea/01b_coral_sea.toml`, day 5, the last: the Japanese men at (-17,-4), 5,116 fit against the 3,444 wanted at balanced. Before: set to attack ("Nothing more is coming and the men that can attack are enough for it"), a fight that day whose losses the scale counted as 320 men in the Allied score and 272 in the Japanese. Now: no order, "1 day of the game is left, fewer than 3 ... the staff begins no attack ashore, because an attack begun so late only costs men". The result moves from 56.58 against 4 points to 53.86 against 0.8: a decisive Japanese victory either way. The hash of day 5 changes.
- `data/scenarios/men-ashore-no-more-coming/109_men_ashore_with_no_more_coming.toml`, days 6 and 7 (2 and 1 left): the attack of day 5 failed, and U and V are set to defend on day 6; no attack on day 7. The hashes of days 6 and 7 change.
- `data/scenarios/stage-3-proof/07a_slice.toml`, days 38 and 39, both takes of the Japanese side: the staff already waited with nothing more coming; its reasons now add the days left. No order, notice or hash changes.

No other game's events or hashes moved; no hash moved where the events did not. Rule 10 changed no game.

**Existing tests that changed.** `tests/scenario_109_men_ashore_with_no_more_coming.rs`, `statement_3_balanced_with_nothing_more_coming_the_units_attack`, the architect's: its assertion that nothing is ordered after day 4 (`g.all_postures()`) now gives `[(4, 1, Attack), (4, 2, Attack), (6, 1, Defend), (6, 2, Defend)]` against `[(4, 1, Attack), (4, 2, Attack)]`, because the game is 7 days and day 6 has 2 left after a failed day. It is ignored with that reason, for the architect to restate. Scenarios 91 and 92 gained the new field and no number.

**Not done.** Statement 5's variant and statement 8 in play. Scenario 105's forty games and 1,000 games were not run again.

## The architect's rulings on the build

Read on Nov 20, 2026, with the build's report and its "As built".

1. **The readings of "As built" stand,** and rows 53 to 55 of `docs/rules/differences.md` are confirmed: a session hands the world the days it is given; "a lift that cannot arrive" is read as one that may not be used by the game's last day, with no passage reckoned; where the judgement waits for a reason of its own, units on attack are set to defend as before.
2. **Three statements of the worked example were the architect's mistakes** and are corrected by him:
   - "no daily hash differs from the same game before this rule" was wrong of a game with last days in it: the rule changes what happens in them. What is true and tested is that the game's length itself moves no hash;
   - the variant in which an attack does not fail and the men stay in cannot be reached in this setting, since an attack that does not fail there takes the place that day. It is tested on the judgement alone;
   - a game of one day never reaches the judgement ashore, and the statement is taken out.
3. **Scenario 109's statement 3 is restated by the architect:** in its game of seven days the men sent in on day 4 are set back to defend on day 6, their attack of day 5 having failed.
4. **Games that moved,** of every scenario file played before and after: the Coral Sea with a person in the Allied seat, where the men are no longer sent in on the last day (still a decisive Japanese victory, 53.86 to 0.8 where it was 56.58 to 4); scenario 109 in its last days; the stage 3 proof's first file in its reasons alone. No hash moved where the events did not, and no golden file was remade.
5. **Found on the way, and left:** in scenario 109 the men ashore are unfed from about day 5 and fire nothing, so every attack after day 4 fails; and an attack is recorded as failed on a day the place falls. Both are the fight ashore's rules and not this one's.
