# 0204: A dry run asked inside a day

Status: a plan by the architect, written on Nov 21, 2026 on the designer's word at the table: a dry run refused at 0600 because "a dry run is asked at dusk" makes no sense to a commander, and it should be played from whatever is known at that hour. It was read by a reviewer who had not seen it written and is amended on that reading; its hand calculation is written next.

Summary: A dry run (a trial, decisions 0175, 0178 and 0181) is refused while a day is open, at 0600 and at 1200. This lets it be asked there. It is played from the state as it stands at that hour, taken as if it were this evening: the first day played is tomorrow, when the order would in truth be taken up, and the hours of today still to come are not played. The paper says so. Playing the rest of today inside the dry run is left as a second stage.

Standing: Planned on Nov 21, 2026; its review folded in; its hand calculation written (scenario 118); rules 1 to 13 built on Nov 21, 2026 ("As built", below); stage 2 (rule 14) not built.

## Why

What is so today:

- **A seat may stop the clock at 0600 and 1200** where the scenario allows it (decision 0156). There it may still state an order to the staff, and that order is taken in at the next dusk: the staff is asked once a day, before the night.
- **A trial asked there is refused twice over.** The session refuses it by its own check, and under that the side's half of a picture is refused "while a day is open, since a picture is of an evening" (decision 0170, 3). Decisions 0175 and 0178 both list "a trial inside a day" as left for later.
- **A preview asked there is not refused.** The session already makes a copy of the game with the open day treated as closed (`as_if_day_closed`, decision 0156, as built), and works the preview on that: the state as it stands, with no pulse still to come resolved and no end of the day run.
- **Nothing flies between pulses.** Strikes in flight and sorties flown are empty outside the air steps of one pulse. What a side has part-way through a day is kept in its state: where a force is on its leg, its fuel, fatigue, groups stood down for the day.
- **A trial builds its own worlds** from a side's view and what it believes, starts each at dusk, and plays whole days. It never touches the game.

So the order a commander weighs at 0600 would be taken up this evening in any case, and the game can already show the staff the state as it stands at 0600 with the day treated as closed. Only the trial is shut out.

## The rules

1. **A trial may be asked while a day is open,** at 0600 and at 1200, by every form it has at dusk: of an order in force, of a draft, beside another, at later dates, and described.
2. **It is begun on the side's view of the copy the preview uses:** the game as it stands at that hour with the open day treated as closed. A trial played and a trial described are both begun on that view. The command state is the same on the copy and in the game, so what is in force and the leaves held read the same on either; it is the day that differs, and the day is the copy's.
3. **The first day played is the day after the open day,** and the commands of the course are handed in on its first morning. That is the dusk at which they would be taken in. On the copy the day counter is the open day, so the session's day for commands, the trial's first day and the dates of a trial at later dates all agree.
4. **The hours of today still to come are not played.** In the trial's worlds each force is where it is at the hour asked, with the fuel it has then. What is left out:
   - at sea and in the air, the pulses still to come: one or two pulses of steaming, search, strikes, and of cargo and fuel worked;
   - ashore, the whole of today. The march, the fight, the day of training or rehearsal, the day's digging in and a base changing side are all worked at the day's end, so a land unit starts the trial a full day behind, and a landing's "when it could go" can come out a day late.
   What the night has already done is in it: supply and feeding, arrivals, replacements and repair.
5. **Orders to units are not in a trial,** as today (decision 0178). The count of orders left out takes in those given for today's pulses as well as those queued for the coming day, and the answer says that the holds such orders make are not in the trial either. A preview does show those holds, so the two differ there.
6. **Hits on men that wait for today's fight ashore are left out,** as a picture leaves them out. Orders a scenario gives inside the day are left out too.
7. **A contact's age is read as the picture holds it,** in pulses.
   - In hours it is counted as if the pulse last resolved were an afternoon, so a contact that has aged through the night is read up to six hours younger than it is. One reported in the pulse just resolved is exact.
   - In the trial's worlds a contact is two pulses younger at 0600, and one at 1200, than it will be at the dusk the trial stands for. It stays inside the age for a strike and the age at which it is dropped that much longer, and that can change what is struck on the trial's first day.
8. **The answer says how it was asked.** What was asked gains `inside_day`: nothing at dusk, and inside a day the day and the hour, as `{"day": 1, "clock": "0600"}`. It is in the answer of a trial, of a trial described and in each row of the list of trials. The staff's words say in one sentence that the trial was asked at that hour, that the rest of that day was not played, and, where the side has men ashore, that what is ashore is a day behind.
9. **The list of trials** says "asked at 0600 of day 1" for such a trial where it says "asked on the evening before day N" for another, and counts a trial asked at an earlier hour of the same day as one asked on an earlier evening is counted today. A trial at later dates says "as things stood at 0600 of day 1" where it says "on the evening".
10. **The same question at the same hour gives the same answer.** The promise that the same question on the same evening gives the same answer is reworded so: asked at 0600 and again at dusk, the answers differ, since the state does.
11. **The game is as it was.** The copy is never resolved and never hashed; the trial's worlds are its own.
12. **The refusal goes:** from the session, from the description of the trial tool given to an agent (`TRIAL_DESCRIPTION`), and from the table.
13. **The table** shows it in the head of the paper, "From the morning of day 2, as things stand at 0600 of day 1", and under the costs, "The rest of today is not played", with "what is ashore is a day behind" where the side has men ashore.

## Stage 2

14. **Playing the rest of today.** Each world of the trial would start with the day open at the hour asked, resolve the pulses still to come with no staff asked, close the day, and go on. It needs a picture that is not of an evening: a day in hand, the hits on men that wait, and ages counted from the pulse last resolved. That changes what a picture is (decision 0170) and is planned on its own, if games show that the hours left out matter.

## Rule card

Reads: the game's state as it stands inside a day, through the side's view of a copy. Writes: nothing in the game. Draws no random number in the game; the trial's own draws are as they are. Adds no state.

## Games on file

No event and no hash of any game moves. No game on file, recorded game or test asks a trial inside a day or rests on the refusal's words. The agent interface's version rises, since an answer gains a field and a refusal is no longer given, and the recorded transcripts are remade for that number alone, as at every rise.

## What it does not do

- It does not play the hours of today still to come (stage 2).
- It does not ask the staff inside a day. An order to the staff given at 0600 is still taken in at dusk (decision 0156, 6).
- It does not change how a picture is made: the half is still taken of a closed day, here a day treated as closed. It does stretch what a picture is said to be: decision 0170 has it "the clock at 1800", and this takes a state of 0600 or 1200 under that name. That is owned here and said on the paper.

## What it costs

- **A copy of the game for each trial asked inside a day,** as a preview costs today.
- **An answer that is wrong in known ways,** all said on the paper: forces at sea start some hours short of where they will be this evening; what today's remaining pulses would cost or change is not in it; contacts are fresher than they will be; and everything ashore is a full day behind.
- **A state never built from before.** A picture's build runs the world's checks, which have only ever run on a day that closed. The build tests a trial at 0600 and at 1200 on the 39 days of the full slice, with men ashore and a bombardment waiting.
- **Records and words that change with it:**
  - decision 0156's line, and the code's own comment on the copy, that it is for the staff step and the check of a command, "nothing else": widened to the view a trial is begun on. The copy is still never resolved further;
  - decision 0181's "in force is read at dusk": at dusk, or at the hour asked;
  - rules chapter 9, the trial, and chapter 8 where it says what may be done at 0600 and 1200; the glossary's "trial";
  - the schemas of the three answers, which take the new field.

## Questions for the designer

None. The designer asked for this at the table; the choice to take the state as it stands "as if it were this evening" first, and to leave playing the rest of today for a second stage, is the architect's, on the size of the two pieces of work.

## How the plan changed on review

A reviewer who had not seen the plan read it against the session, the trial, the picture and the day's resolution on Nov 21, 2026. The architect checked the main findings against the source.

- **Orders to units.** The plan had them handed in "as the preview does". A trial never tries orders to units (decision 0178), and those given for today's pulses were not even counted as left out. Rule 5.
- **Ashore.** The plan said "some hours" were lost. The land units' whole day is worked at the day's end, so ashore a full day is lost. And supply is the night's work, already done, not the day's end's. Rule 4.
- **Contacts.** The plan had the error in hours on the wrong contacts and left out that the ages in pulses are younger than they will be at dusk. Rule 7.
- **A trial described** would not have been refused on the game itself and would have started a day early. It is begun on the copy too. Rules 2 and 3.
- **The field, the list of trials, the promise of the same answer, and the places to change** are named. Rules 8 to 10 and 12.

## As built

Rules 1 to 13 are built on Nov 21, 2026. Rule 14, stage 2, is not.

**Where.**

- `crates/sim-agent/src/session.rs`: the refusal "a trial is asked at dusk" is gone from `trial_tool`. A new `Session::closed_world` gives the game itself at dusk and `World::as_if_day_closed` inside a day; `trial_tool` reads "in force" and the leaves held on it, and `play_trial` and `describe_trial` are handed it and begin the runner on its side's view (rules 2 and 3). The count of orders to units not tried is the seat's queue for the coming day and its orders given for the day's pulses together (rule 5). `trials` gives each row `inside_day` and its words (rule 9).
- `crates/sim-agent/src/trial.rs`: what was asked gains `inside_day` (rule 8) and whether the side has men ashore. The sentence of rule 8 follows the line "Tried" in a trial's, a window's and a described trial's words. The orders line inside a day says they and the holds they make are not in it. A window's line of the enemy on the first eve says "as things stood at 0600 of day 1" in place of "on the evening before day 2" (rule 9).
- `crates/sim-cli/src/seat_trial.rs` is unchanged: it reads the first day and the dates from the view it is handed (`view.day() + 1`), which on the copy is the open day, so day 2 comes out with no change there (rule 3).
- `crates/sim-cli/src/staff_tools.rs`: `TRIAL_DESCRIPTION` says a trial may be asked inside a day and what it then leaves out, rewords the promise (rule 10) and drops "refused at an open day" (rule 12).
- `crates/sim-core/src/resolve.rs`: the doc comment of `as_if_day_closed` names the trial's view among its uses. Decision 0156's and 0181's lines are widened as "What it costs" says.
- The schemas: `trial.schema.json`'s `asked` gains `inside_day`, which `window` and `trial_described` take by reference, and `trials.schema.json`'s row gains it. The agent interface is version 40.
- `table/index.html` (rule 13): the head of the paper and the line under the costs, and `cannotTry` loses its sentence for a refusal at mid-day. The staff's report does not repeat the sentence of rule 8.
- The rulebook: chapter 9, A seat's trial, rules 6, 8, 11 and 13 and a new rule 14; chapter 1, At the tools and A world built from a picture, item 2 (where what may be done at 0600 and 1200 is said; chapter 8 says it of the override alone, and gains a line there); the glossary's "Trial".
- Scenario 118: `data/scenarios/a-dry-run-inside-a-day/`, with a placeholder map, a file of sides (`class_sides_placeholder_scenario_118.csv`, and its frozen copy) and the trial's figures of scenario 96; its tests in `tests/scenario_118_a_dry_run_inside_a_day.rs`.

**Readings.**

1. *The words of rule 8.* "Asked at 0600 of day 1, from things as they stand at that hour: the rest of day 1 was not played." with ", and what is ashore is a day behind" before the stop where the side has men ashore; a trial described says "is not played". "Men ashore" is a land unit of the side's with men ashore, read on the copy.
2. *The orders line.* At dusk it is as it was, word for word, so no answer at dusk changes. Inside a day: "You have 1 order to units given for the day's pulses or queued for the coming day: a trial does not try orders to units, and they and the holds they make are not in it."
3. *The list of trials.* A trial asked inside a day keeps the standing `earlier_evening` once the clock has moved, as rule 9 says, and its note is "asked at an earlier hour", wherever the clock now stands; one asked at this stop is "asked at this hour, on the queue as it stands". The row's `inside_day` is a copy of what was asked's.
4. *The interface's version.* The schemas' README says an added field does not raise the version; the plan raises it, since the refusal is no longer given, and it is raised.
5. *The copy at dusk.* At dusk `closed_world` borrows the game and makes no copy, so a trial at dusk costs what it did.
6. *Scenario 97.* Its test `refused_while_a_day_is_open` rests on the refusal, against the plan's "no test rests on the refusal's words". It is kept, ignored with the reason that decision 0204 supersedes it; scenario 97's doc comment is not touched.

**What did not hold.**

- *Statement 7 cannot arise as set.* Order 1, stated at the dusk before day 1, is taken in at the start of day 1 and so is in force at 0600, and an order in force has no later date (decision 0181, amended 5): the window is refused. Its test stands as written and is ignored; `statement_7_at_later_dates_as_built` checks that refusal and tries instead a second order queued at 0600, whose window's first date is day 2.
- *Statement 13's "at both"* is read as in the game and in the trial: a contact reported in the night just resolved is 0 pulses old in the game's list at 0600 and is read as 0 hours by the trial. At 1200 the same contact is a pulse old, the morning having passed with no sighting.
- Every other statement and property holds as written. The property "a state never built from before" is tested on the 39 days of `stage-3-proof/07a_slice.toml`, with orders inside a day and the trial's figures added, at 0600 and 1200 of day 18, when Allied men are ashore at Lunga and the hits of the night's bombardment by Japanese task force 6 wait for that day's fight ashore; the trial is answered at both.

## The architect's rulings

Read on Nov 21, 2026: the session's changes in full, the answer's words, and the build's account of what moved.

1. **The build is what the plan asked for, and small.** A trial is begun on the copy a preview already uses; the runner needed no change, since on that copy the day counter is the open day and the first day played comes out as the day after.
2. **Statement 7 is corrected, not the code.** An order in force has no later date to try, and order 1 is in force by 0600. The statement now says that it is refused for that reason, and tries at later dates a second order stated at 0600. The test the build wrote is of that. The correction is dated in the test file.
3. **The property that the game is as it was** is tested with a trial and a trial beside another, where the build first tried a trial at later dates and met the same refusal. The property never named a form of trial; it holds.
4. **Scenario 97's statement that a trial is refused while a day is open is superseded by this decision.** Its test is set aside with that reason and its doc comment carries a dated note. The plan said no test rested on the refusal's words; one did, and the review missed it.
5. **The interface's version is raised** although one added field alone would not raise it, because a refusal an agent could meet is withdrawn.
6. **A trial asked earlier in the same day is listed as "asked at an earlier hour",** under the same standing as one asked on an earlier evening. One standing serves both: in either case the state has moved on since.
7. **Statements 3 and 8 are tested by building the trial's worlds from the copy's view** with the trial's own routines, since an answer does not show its worlds. Statement 2, the first day, is what shows that the session uses the copy. That is accepted.
8. **Not seen by the build:** the paper at 1200, the paper of a trial at later dates asked inside a day, and the paper at dusk after the change. The words under test are the answer's; the table's are checked by its screen checks and by one look at 0600.

