# 0141: Units made ready, and a base built up: step 2 of stage 3

Status: settled by the architect on Nov 17, 2026, inside the plan for step 3, stage 3, that the designer confirmed the same day (decision 0139, step 2 of its eight). It writes two tasks and their doctrines over rules already built (decisions 0095, 0097, 0098, 0099 and 0103), with the hand calculation. It is built after decision 0140. Built on Nov 17, 2026: see "As built".

Summary: Step 2 of stage 3: two tasks a commander may state and that "take a place" will form: "ready a landing", by which the staff gives a force its landing order and roster and sets each unit to train or to rehearse by its experience, its risk and the date, and says when the landing is ready; and "build up a base", by which it sets the working units, and any fighting unit it is given, to the jobs wanted, in order, and says how many days each will take

Standing: Settled by the architect, Nov 17, 2026, inside decision 0139. Built; its readings await the architect (As built)

## Why

The rules by which a landing is prepared are built: a force gains loading for each day in port with its roster, a unit gains rehearsal for each day it stands ready on that roster, a raw unit gains experience by days of training, and working units build a field, a port and defences. Each is set by an order a scenario gives by hand. The spec asks that nobody give a `set_activity` order by hand ("The operation", step 2). These two doctrines give them.

They are the first phase of "take a place" (decision 0139, "An operation is a tree with phases"). They are written first, as tasks a commander may state himself, as cover from the air was before "hold a place".

## Ready a landing

1. **A task:** it names a task force, the coast hex of the landing, and, in its forces limit, the land units that are to land. It is addressed to the task force. It may have a date: the morning by which it is to be ready.
2. **The force's order.** The staff gives the force `prepare_landing` for the hex with the task's units as its roster, where the force's preparation differs.
3. **Each unit, each morning,** is set to train or to stand ready, which on a roster is to rehearse:

| The risk | A unit below its service's standard of experience |
| --- | --- |
| Preserve the force | Trains until it is at the standard, then rehearses, whatever the date |
| Balanced | Trains until it is at the standard, or until the days left before the date are no more than the days its rehearsal still needs; then rehearses. With no date it trains to the standard first |
| Press | Does not train. It rehearses from the first day |

A unit at or above the standard rehearses. A unit that cannot train (it has men aboard, or it is at no base of its side) rehearses. The days a rehearsal needs are what is left to 100 over the day's gain for that unit, rounded up.
4. **A working unit is given nothing by this task.**
5. **When it is ready.** The task is ready on the first morning that:

| The risk | The force's loading | Each unit's rehearsal | Each unit's experience |
| --- | --- | --- | --- |
| Preserve the force | 100 | 100 | At the standard |
| Balanced | 100 | 100 | Any |
| Press | 50 | 50 | Any |

The staff hands in a notice that it is ready, once. Step 3 of this stage asks leave to launch on it.
6. **A date it cannot make.** On the first morning the staff can see that the task will not be ready by its date, it hands in a notice with the days it will be late and what binds: the loading, a unit's rehearsal or a unit's training.
7. **Reasons** carry the numbers: each unit's experience against the standard, its days of training counted against the days a level takes, its rehearsal and the days it still needs, the force's loading, and the days to the date.

## Build up a base

8. **A task:** it names a base and what is wanted there, in order: an airfield; its damage mended; defences to a stated number of points; the port worked; aircraft repair. It is addressed to the base. With no forces limit it may use the working units at the base. A forces limit may add fighting units, which the staff then puts to work; it puts no fighting unit to work that it was not given.
9. **Each morning** every unit the task may use, that is at the base and not held, is given the job of the first want not yet met, where its job differs. All work the same job.
10. **A want is met** when the base has an airfield; when its airfield has no damage; when its defences are at the number or above. The port and aircraft repair are never met: they are worked while they are the first want left.
11. **With every want met** the staff gives no order, says so, and hands in a notice that the task is met, once. A want that stops being met, a field damaged again, is worked again.
12. **Reasons** give the work still to do on the want, the work the units make in a day, and the days that comes to.

## What both keep

13. **A unit serves one task.** Where two tasks may use a unit, the higher in priority has it, and the other's reason says so.
14. **A hold is honoured,** and a commander's order to a unit a task may use is an override that holds it, as for an air group (decision 0115, 3).
15. **Nothing is ordered without leave to prepare.**
16. **A unit before the day it may be used from** is left out (decision 0140).

## What it leaves for later steps

- **Choosing which units land,** by what each is good at: step 3.
- **Shipping gathered for the landing:** step 3.
- **What a base needs for an operation,** worked out by the staff and not stated: step 3 forms this task with its wants.
- **Training when no landing is planned.**

## Rule card

- **Reads:** a unit's experience, days of training, rehearsal, activity, job, where its men are; a force's preparation; a base's airfield, damage and defences; the ground figures; the risk and the date of the task.
- **Writes:** two kinds of intent and what a "build up" task wants, in the command state; nothing else.
- **Reuses:** the orders `prepare_landing` and `set_activity`, the day's activity, and every rule of decisions 0095 to 0099 and 0103 as they are. A forces limit gains land units.

## What it costs

- **State:** the two kinds of intent, the wants of a build-up, land units in a forces limit, the date of a task, in the hash and named. The state layout and saved game versions rise; the doctrine version rises. The rules label does not.
- **The staff gives orders to land units for the first time.**

## The hand calculation

`tests/scenario_66_units_made_ready.rs`.

## As built

Built on Nov 17, 2026, on the branch `stage3-step2`. Nothing is tuned and no figure changed. Every number of the hand calculation is asserted exactly and none is ignored: no mismatch was found. Every scenario built before this one gives the events it gave: the run baselines changed in their hash lines only, and the recorded games in their versions and day hashes (one transcript also in new keys of `command_state`, which are null for its intents).

### What was built

- **The state (`crates/sim-core/src/command.rs`).** `IntentKind::ReadyLanding { force, hex, by_day }` (code 11, `ready_landing`, addressed to the force) and `IntentKind::BuildUp { base, wants }` (code 12, `build_up`, addressed to the base), both possible roots and standing under nothing yet; `Want` (airfield, mend, defences to a number, port, aircraft) and `Wants`, one to five, each kind once, defences from 1 to 100, made only by `Wants::new`; `ForcesLimit::land_units` and `CommandState::land_units_on_chain`; `HoldTarget::LandUnit` (code 5); `NoticeKind::Ready`, `Late` and `Met` (codes 5 to 7); `CommandRefusal::NotYourLandUnit` and `NotACoastHex`, at the end.
- **Intake (`command/intake.rs`, `command/hold.rs`).** A ready-a-landing's hex must be a coast hex on the map and its force the side's; a forces limit's and a hold's land units must be the side's; a limit on a sub-intent may name only land units its parent may use. `World::may_use_land_unit` is the one test of what a task may use, read by intake, the override and the narrowing: a unit named in every forces limit on the chain; with none, a build-up's working units with men ashore in its base's hex. A staff's order to a land unit is inside the limit by that test, and is refused as held when the unit is held. A commander's order of any kind to a land unit some intent in force may use holds it (the override). A pure helper, `sim_core::works::work_to_raise_defences`, gives the work from one level of defences to another at each grade's cost.
- **The doctrine (`crates/sim-staff/src/ready.rs`).** `Day::readies`, run last in the air doctrine's morning (`air.rs`), serves both kinds in one priority order. The leaves are the air doctrine's (`air.rs`, `leaves`), with the two kinds stopping at preparing.
- **Scenario 66** (`data/scenarios/66_units_made_ready.toml`, map `data/map/placeholder_units_made_ready.toml`) and its 19 tests: the hand calculation in five, and each property in its own.
- **Versions.** State layout `flagplot-state-v40`; saved game 15; agent interface 18 (and the schemas' constants); doctrine `supply-21`. The rules label stays 18: no game without either task plays differently.
- **The tools.** `set_intent` takes `ready_landing` (`task_force`, `hex`, optional `by_day`) and `build_up` (`base`, `wants`, and `defences` where it is a want); a forces limit takes `land_units`; `hold` and `release` take `land_unit`; `command_state` gives `by_day`, `wants`, `defences`, `land_units` and `land_units_binding`; the notices are `ready`, `late` and `met` (`docs/commands.md`).

### Readings taken

Where the decision leaves a detail open; the architect may rule on each.

1. **Leaves.** Both kinds take leave to prepare as a hold does and work while their plan is preparing or released. They never ask leave to launch, hand in no `waits_for_leave_to_launch`, and the staff never releases them (item 5: step 3 asks leave to launch on the landing).
2. **The standard is 2,** the service's standard (decision 0097, 1), a constant. A unit below it whose experience is at the training ceiling cannot be trained higher and is counted with the units that cannot train: it rehearses.
3. **"Plans without it."** A held unit, a unit before its day, a unit a higher task took that morning and a unit with no man left are left off the landing's roster and out of its readiness, and named in its reasons. So a unit released or come to its day is added to the roster then, and the force's loading is cut once by `preparation_kept_on_change`, as the property "Before its day" has it.
4. **A working unit named in a landing's forces limit** is put on the roster (it is to land: item 1), given no order (item 4), and not counted in the readiness, since it never rehearses.
5. **A build-up with a forces limit** uses the land units the limit names, read up the tree as every forces limit is (decision 0117, 7), working or fighting; "a forces limit may add fighting units" is read so, as the property "A fighting unit put to work" names unit 3 in the limit beside unit 2. With no limit it uses the working units at the base.
6. **"At the base"** is men ashore in the base's hex; a unit with none there is left out with words.
7. **"Its damage mended"** is met at a base with no airfield: there is nothing to mend.
8. **"Once"** is read from the side's own `StaffNotice` events: no notice of a kind is handed in for a plan that had one before. Each task here is a root with its own plan.
9. **Ready** needs the force's preparation already for the hex with the roster the staff would give (no landing order that morning) and a roster that is not empty, beside item 5's table, read over the fighting units on the roster.
10. **The projection for "late"** takes the loading the force will have after the morning's order (0 for a new hex, cut where a unit is added) and gains `preparation_per_day` a day; each fighting unit's days of training by the rule of item 3 (at balanced with a date, no more than the days left less the days of rehearsal to 100) and then of rehearsal to the risk's mark. The morning it is ready is today plus the longest; what binds is the longest term, the loading first and then by unit at a tie, a unit's "training" where its days include training and else its "rehearsal". A unit that can never be ready (at preserve the force, below the standard and unable to train; or one that gains no rehearsal) makes the task late with no count of days.
11. **"Days left before the date"** is the date's day less today's.
12. **The notices `ready`, `late` and `met`** fire no standing interrupt: the decision names none. Each rests on a no-order reason of its own; a morning with no order for a task has one reason that says so.
13. **A held force** is given no landing order; its units are still set to train or stand ready.
14. **Numbers in words** are written as the fixed-point numbers print, with no thousands mark: "4000 of work to do on an airfield, 1000 a day by the units at it, 4 days".
15. **The properties' choices.** "A hold and an override": the commander's order of day 2 sets unit 2 to train; the staff, which would set it back to stand ready, gives it no order after. "What the Allied side cannot see": a Japanese land unit at (6,0) of 100 men in one world and 300 in the other. "Every scenario built before this one gives the events it gave" is shown by the run baselines and the recorded games, as above, and by a test that scenario 66 with its commands cut hands in nothing.

### Earlier tests touched

No expected number changed. `tests/scenario_41_hold_change_release.rs` and `tests/scenario_45_soak_ashore.rs` gain the arm `HoldTarget::LandUnit` in a match (counted as a setting is). Scenario 66 is added to the scenarios exempt from three walks of the folder: `tests/scenario_04_experience_values.rs` (it gives its placeholder land units experience) and the two run-without-a-staff hash checks of `tests/scenario_06d1_pilots_and_replacements.rs` and `tests/scenario_06d2_base_supply.rs` (it is played by the staff, as scenario 65 is). `crates/sim-cli/tests/export_pictures.rs`'s ten export hashes are remade at layout v40, as at v39. Version pins move in the agent's and the saved games' tests (18 and 15), and the sim-core command tests' forces limits gain `land_units`.

### Files to read

- `crates/sim-staff/src/ready.rs`: the doctrine whole.
- `crates/sim-staff/src/air.rs`: `is_air`, the leaves' stop at preparing, and `readies` in `Day::run`.
- `crates/sim-core/src/command.rs`: the kinds, `Want`, `Wants`, `land_units_on_chain`.
- `crates/sim-core/src/command/intake.rs`: `may_use_land_unit`, `check_intent_kind`, the forces limit, the hold and the guard; `command/hold.rs`: the override.
- `tests/scenario_66_units_made_ready.rs`.

### The architect's rulings, Nov 17, 2026

The fifteen readings are accepted as built. Three are worth their reason.

- **Reading 1, the leaves.** A task that readies units never sails anything, so leave to launch is not its to ask. The end it stands under asks it (decision 0142).
- **Reading 5, a build-up's forces limit.** The units a limit names are the units used, working or fighting. That is what a limit means everywhere else.
- **Reading 12, no standing interrupt.** Ready, late and met are news of a plan and not alarms.

The viewer reads the two kinds (`viewer/index.html`, `viewer/check.js`): each has its words, its addressee and its ring on the map.
