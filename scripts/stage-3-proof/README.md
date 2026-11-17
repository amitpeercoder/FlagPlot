# The scripts that read stage 3's proof

The proof of stage 3 is planned in decision 0153: the 39 days of the staff's setup, staff against staff, on seeds 1 to 1,000, in three arrangements. These scripts read its conditions from the umpire's files. They were written and tested before the proof was run, so that the reading is fixed first, as decision 0128's were.

They are the umpire's. They read the truth: `sim-cli export` files and `sim-cli batch` files. Nothing that plays the game reads them or what they print (`docs/engineering/truth-and-belief.md`). They print counts, medians and comparisons, never a verdict of pass or fail: the marks are read by the architect.

Python 3.9 or later, standard library only. No build step and no test runs them; they are run by hand.

## The scripts

| Script | Condition | Reads |
| --- | --- | --- |
| `conditions.py` | 2, 3 and 4 | The exports of one arrangement, one file a seed; with `--batch`, that arrangement's batch file as a check on condition 2 |
| `margin.py` | 5 | Two batch files: a side from its intents, and the same side given nothing |
| `same.py` | 1, its second half | Two batch files of one arrangement, and two folders of exports of the same seeds |
| `pick.py` | 6, the picking rule | The exports of arrangement (a), seeds 1 to 10 |
| `common.py` | | Shared reading of the files |

## How to run them

Run `sim-cli` from the top of the repository, so that both runs of a batch name their inputs by the same paths. Keep the files out of the repository.

```
cargo build --release -p sim-cli
S=data/scenarios/...            # the arrangement's scenario file
OUT=/some/folder/outside/the/repository
for n in $(seq 1 1000); do target/release/sim-cli export $S --seed $n --out $OUT/a/seed$n.json; done
target/release/sim-cli batch $S --runs 1000 --out $OUT/a.json
target/release/sim-cli batch $S --runs 1000 --out $OUT/a.again.json

python3 scripts/stage-3-proof/conditions.py $OUT/a --batch $OUT/a.json --list --jsonl $OUT/a.conditions.jsonl
python3 scripts/stage-3-proof/margin.py $OUT/a.json $OUT/b.json      # the Allied half: (a) beside (b)
python3 scripts/stage-3-proof/margin.py $OUT/a.json $OUT/c.json      # the Japanese half: (a) beside (c)
python3 scripts/stage-3-proof/same.py --batch $OUT/a.json $OUT/a.again.json --exports $OUT/a.sample $OUT/a.sample.again
python3 scripts/stage-3-proof/pick.py $OUT/a --out $OUT/legibility.json
python3 scripts/stage-3-proof/pick.py --rows                        # the matchers, and which are tested
```

`conditions.py --base N` names the base whose hex is "the hex of Lunga" (1 by default, the record's); another base is only for trying the script on another scenario. `pick.py --as-seed-1 FILE` reads one export as seed 1 whatever its seed, to try the matchers on a scenario of main. An export takes about 0.1 s to read.

## From the record's words to the file

Every count is in seeds: a seed counts once however many events it has. L is the hex of base 1 in the export's `bases`, (2,-4). Event fields are in each event's `data`.

| The record says | The script reads |
| --- | --- |
| 2. An `order_rejected` for an order a staff gave | `order_rejected` whose `order` is, in the export's `orders`, an order with `issuer` `staff`. Orders with `issuer` `player` (scripted or the commander's) and `local_commander` are counted apart |
| 2. A `command_refused` for a command a staff gave | `command_refused` with `issuer` `staff` |
| 2. A commander's command that is refused, counted apart | `command_refused` with `issuer` `player`. `local_commander` (command levels) is counted apart again |
| 2, as a check | The batch file's `per_run[].refusals`: `staff_orders_not_as_held` plus `staff_orders_other` against the staff orders rejected in the events; `commands` against every `command_refused` (the batch does not split commands by issuer) |
| 3. A `men_landed` of an Allied unit in the hex of Lunga | `men_landed` with `hex` L whose `unit` is Allied in the land units' records (the `task_force`'s side where no record names the unit) |
| 3. A `base_changed_side` of base 1 to Allied | `base_changed_side` with `base` 1 and `to` `allied` |
| 3. Supply in a `cargo_unloaded` at an Allied base in the hex of Lunga on a day after the first Allied man landed there, from a force other than the one that landed him | `cargo_unloaded` with no `cargo` key (supply; fuel has `cargo: "fuel"`) and `unloaded_t` above 0, at a `base` whose hex is L (base 1, or a base a landing formed there, by its `base_formed`) and whose holder at that event is Allied (the start's holders, then each `base_formed` and `base_changed_side` in event order), on a `day` greater than the day of the first Allied `men_landed` in L, with a `task_force` other than that event's |
| 3. A search, a patrol or a strike by an Allied group based at base 1, read as decision 0128 read them | A search: `search_flown` with `base` 1 by an Allied `air_group`. A patrol: `air_group_fatigue` with `change` `gained` and `sortie` `patrol`, in the morning or the afternoon, by an Allied group at base 1. A strike: `strike_launched` or `base_strike_launched` by an Allied group at base 1. A group's base is its record's `base` in the state at the end of the step before the event (the export's `steps`); the side is the group's record's `side` |
| 4. A `men_landed` of a Japanese unit in the hex of Lunga on a day after the first Allied man landed there | `men_landed` with `hex` L whose unit is Japanese, on a `day` greater than the first Allied landing's in L. With no Allied landing in L, none counts |
| 4. A `ground_fight` in the hex of Lunga in which the Japanese side attacks, after base 1 is Allied | `ground_fight` with `hex` L and `japanese_attacks` true, where base 1's holder at that event is Allied. Printed beside it: base 1 Allied at any event before the fight |
| 4. A `shore_bombardment` of base 1 by a Japanese force | `shore_bombardment` with `base` 1 whose `task_force` is Japanese |
| 4. A Japanese force on the bombardment or the surface combat mission that ends a night pulse in the hex of Lunga | In the state at the end of each `night` step of the export's `steps`, a Japanese task force with `mission` `bombardment` or `surface_combat` and `position` L (the hex whose centre is nearest) |
| 5. The median of the Allied margin; of the Japanese, its negative | The batch's `per_run[].allied_margin`, a decimal string read exactly; the Japanese margin its negative. The median of an even count is the mean of the two middle values, which are printed beside it |
| 1. The batch files are the same, and the exports of a sample of seeds | Byte for byte. Where two differ, the first seed and key (batch) or the first day hash and event (export) that differ are named |

`conditions.py --jsonl` writes, for each seed, the event numbers each count rests on, so that any count can be checked against `sim-cli events`.

## The picking rule for legibility

The rule is in `pick.py`'s docstring. In short: a reason is a `staff_reason` event; it rests on an order when `data.order` is set, and on a staff command when one of its causes is a `command_taken` or `command_refused` with `issuer` `staff`. For each doctrine of the record's table, in its order, the first four reasons that rest on either and that a row of the doctrine matches are taken, in event order on seed 1, both sides together, then seed 2, 3 ... up to 10. A doctrine with fewer than four on the ten seeds is printed with what it has and "fewer than four on the first ten seeds". Each reader is given `prompt_text`: the intent served (with the intents above it) and the reason's evidence, binding, worked and would-change parts.

The matchers are one table, `MATCHERS`, and a doctrine's reasons are changed there and nowhere else. A reason that rows of two doctrines match is taken by each, and flagged. No reason is skipped for having the same words as one already taken: decision 0153 does not say to, as decision 0128's rule did.

| Row | Doctrine | Matches | Tested on |
| --- | --- | --- | --- |
| 1a | Units made ready | An order serving "ready a landing" | Scenarios 66 to 69, 72, 73 |
| 1b | Units made ready | An order serving "build up a base" | 66 |
| 2a | The force chosen | The staff's `form_intent` of "ready a landing" under "take a place" | 67 to 69, 72, 73 |
| 2b | The force chosen | The staff's `set_limit` on that intent on the morning it is formed | 67 to 69, 72, 73 |
| 2c | The force chosen | The staff's `set_limit` on that intent on a later morning: a later lift, or a choice changed by a look at the place | 68 (a second lift), 72 (a look) |
| 3a | Ashore | A posture order to a land unit under "take a place" | 67, 68, 72, 73 |
| 3b | Ashore | A posture order to a land unit under "hold" | 69 |
| 4a | The supply that follows | An order or staff command serving a "keep supplied" with a hold above it | 69 |
| 4b | The supply that follows | An order to a task force serving a hold itself: a force with men aboard sent by a hold | **Untested**: decision 0152 is not on main |
| 5a | By night | A speed or route order to a task force whose reason says the run "goes by night" | 70 |
| 5b | By night | A route order to a task force serving "bombard" | 71 |
| 6a | Cover and reconnaissance | The staff's `form_intent` of a carrier task ("operate the carrier force") under "take a place" | 73 |
| 6b | Cover and reconnaissance | The same under a denial, with "island" in the intents above it | **Untested**: decision 0151 is not on main, and the words of "control an island" are not known yet; confirm the row when it lands |
| 6c | Cover and reconnaissance | A base strike order (`air mission base_strike`) | 73 |
| 6d | Cover and reconnaissance | A reconnaissance order (`air mission reconnaissance`) | 72 |

## What it was tested on

On files made from main on Nov 17, 2026, kept outside the repository, with scripted games standing in for the staffed ones: scenario 7 (seeds 1 to 6, and a batch of 20 run twice); the staff's setup `data/scenarios/staff/07_full_slice.toml` with no intent (seeds 1 to 3, a batch of 20); a copy of the staff's setup with intents of the kinds on main added (take Lunga and Tulagi, hold base 1, a Japanese hold of base 1, a Japanese take of Lunga from day 12, and a Japanese bombardment of base 1 refused on day 1 and accepted on day 18; seeds 1 to 10, a batch of 20; not committed); scenario 45, the soak ashore (seeds 1 to 3, for the Japanese bombardments of base 1); scenarios 66 to 73 for the matchers; and one export edited by hand to hold a staff order rejected and a staff command refused, the only way to show those counted, since no game on main has one. Each count was shown counted on some seed and not counted on another; condition 2's staff counts were shown counted only on the edited file.
