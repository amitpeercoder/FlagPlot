# 0238, as built: Switch 4 measured, and kept for now

Nothing is built. This file keeps what the architect worked out on Nov 27, 2026, so that it is not worked out twice.

## The statements as they would be restated (Nov 27, 2026): not in force

Written by the architect against the measurement, each checked against a run with the key forced on. They are **not** in the test files and no test asserts them: the tests state the game as it is, with the key. When switch 4 is retired the architect reads these again against the code of that day before they go into the doc comments. Lines are the doc comment lines that would be added, by file.

### `tests/scenario_130_a_limit_binds_what_it_names.rs`

```

## Restated for decision 0238 (the staff's choice of a source is the one rule)

Restated by the architect on Nov 27, 2026, before the code. The rules
are in the rulebook, chapter 8, The staff chooses a source of supply.
Where a statement above and this section differ, this section holds.

The holds' game (`130_two_holds.toml`, part E's doc comment): base 1 at
S, (0,0), with 1,000 t and no man; base 2 at T, (5,0), and base 3 at U,
(0,5), each with 10 t and 200 men who eat 200 x 0.005 = 1 t a day; task
force 1, one fast transport, stopped at base 1 with nothing aboard.

5, its last limb (a limit that names no source). The first writing had:
   with the hold's forces limit naming only land unit 1, no supply is
   formed and the reason says no source is named. Now the staff
   chooses. In statement 5's game both units, 400 men, are at T and
   none is at U. On day 1, for the hold of base 2 (intent 1) with a
   forces limit of land unit 1 alone: base 1 keeps nothing and can
   spare 1,000 t; the place's need is 15 days of 400 x 0.005 = 2 t a
   day, 30 t, which base 1 covers; task force 1 lies there with nothing
   aboard and no limit on the hold's chain binds ships; it is 200 nm
   from T. Base 3 keeps nothing (no man is at U) and can spare 10 t,
   less than the need; no force lies there; U to T is off the axes,
   charged its straight line, 200 nm. The source is base 1: one that
   covers the need comes first. The staff forms "keep base 2 supplied"
   under intent 1 and sets its forces limit to source base 1 and task
   force 1. So: a limit that names a land unit binds land units and
   nothing else, and the source and the ships, not named, are chosen by
   the staff. The limb with `source = 1` stands, and there the supply's
   forces limit, which the staff sets, is source base 1 and task force
   1 too: the source his, the ships the staff's.
7 and 24. The games on decision 0238's list of games that move have
   their rows of `BEFORE` remade once, by the build, and
   `69_taken_and_held.toml` and the other files of that
   list with a forces limit on another kind of order are added to
   `MOVED`. A file not on that list whose digest moves is a stop.
18, the source. The first writing took scenario 69 with the hold's
   limit naming no source and a base 3 at C, where the staff said that
   no source is named. That reason is no longer given: there the staff
   chooses base 3 (scenario 69's own restated property), and no reason
   of that game speaks of a source. The game is now scenario 132's
   file c (bases 1 and 2 alone, as that test makes it from its file),
   in which the staff chooses base 2 and base 2 comes to have nothing
   to spare. On each morning the supply (intent 2) cannot be met for
   want of what the source can spare, its reason says the staff chose
   the source and that no other base of the side can spare supply, and
   carries the source limit on the order it is set on: intent 2, the
   supply the staff formed, whose forces limit names the source. At
   least one such morning is found in the ten days.
20. The pinned hash and digest of each game it lists that is on
   decision 0238's list are remade once, by the build; the statement
   stands.
21 to 23. They stand as written. Each supply the staff forms under a
   hold now has a forces limit of its own, set by the staff the morning
   it is formed (source base 1 and task force 1, the commander's), so
   each game has two staff commands more on day 1; the first route and
   its day are as stated.
```

### `tests/scenario_36_one_convoy.rs`

```

# Restated for decision 0238

Restated by the architect on Nov 27, 2026, before the code (the staff's
choice of a source is the one rule: rulebook, chapter 8, The staff
chooses a source of supply). Where the text above and this section
differ, this section holds.

- **An approved plan whose pool has no ship afloat, where the
  commander's limit names no ship.** The first writing had the plan
  marked failed on day 3 and its ships freed on day 4. The limit of day
  3 names a source and no ship or task force, so it binds the source
  and nothing else, and the ships are the staff's to find (Plans): an
  empty pool does not mark the plan failed. On day 3 the staff hands in
  no command and no order, and one reason, serving amendment 1 of plan
  1: no ship of the pool of intent 1 is afloat, so there is nothing to
  send. Plan 1 is released at the end of the game and never failed or
  complete. On each day from 4 to 18 the staff hands in no order and no
  command, `free_ships` among them, and that one reason. The force at
  sea completes its run: 280 t landed, as before. The test's name says
  "is not marked failed" for "is marked failed".
- **A plan never approved** with no ship afloat: as written.
```

### `tests/scenario_69_taken_and_held.rs`

```
| 6 | 120 | For intent 2: forms intent 4, keep base 2 supplied, under intent 2, with a floor of 5 days, and sets intent 4's forces limit to source base 1 and task force 1, the commander's source and his ships (decision 0238). Marks plan 1 complete; notice: intent 1 is met. For intent 2: units 1 and 2 set to defend | Ship 1 lands 44 t in 2.2 hours. The force steams for A at hour 132 |
Restated by the architect on Nov 27, 2026, for decision 0238 (the
staff's choice of a source is the one rule: rulebook, chapter 8, The
staff chooses a source of supply, Under a commander's hold). The staff
sets the forces limit of the supply it forms under a hold in every
case. So the commands of the game that do not prepare or release a
plan are, in order: day 1, form intent 3 and limit its forces; day 6,
form intent 4, then `set_limit` of intent 4's forces to source base 1
and task force 1 and no ship, then mark plan 1 complete; day 9,
`add_run` and `commit_ships`. Seven, where the first writing had six.
Plan 2 is amended once, on day 6, by that limit. The reason of the
forming says the commander's forces limit names base 1 as the source.
`command_state` gives intent 2 `supplied_from` base 1, `chosen_by`
`commander`, from day 6. No order, load, passage, ton or hour of the
hand calculation moves: the source and the ships are the ones the
hold's limit named, and a source the commander names gives down to its
reserve, as before. The same command is added on the same morning in
every property below in which intent 4 is formed.

- **A hold with no source, and no base to choose.** Restated by the
  architect on Nov 27, 2026, for decision 0238; the first writing had
  the reason of decision 0222, rule 1. With intent 2's forces limit left
  out: the load of day 2 is 90 t. On day 6 units 1 and 2 are set to
  defend and the staff weighs the bases to choose a source. Base 1 is
  the only base of the side not in the place's hex, and the staff may
  not take it: it is the source of intent 1, an intent higher in
  priority, which stays in force when its plan is complete. Base 3,
  the landing's, is in the place's hex and is not weighed. So no base
  may be chosen, no supply intent is formed on any day, and the hold's
  reason of day 6 says that the staff chooses a source, that no other
  base of the side can spare supply, names base 1 as the source of
  intent 1, and says base 2 lives on what it has: 1 day of supply
  today (the hex holds the 10 t base 3 was given on the afternoon of
  day 5, against 6 t a day: 1.67, cut down). It would change with "a
  base of the side with supply to spare", and it offers no source to
  name. Ten orders in all, as before.
- **A hold with no source and a base the staff chooses.** Restated by
  the architect on Nov 27, 2026, for decision 0238; the first writing
  had no supply formed and base 3 named as a base a source may be named
  among. As above, with an Allied base 3 at C, (0,6), holding 500 t
  and no reserve (the landing's base is then base 4). On day 6 the
  staff weighs: base 1 is passed over, the source of intent 1. Base 3:
  no man eats at C, so it keeps nothing and can spare 500 t; the
  place's need is 15 days of the 1,200 men at B, 15 x 6 = 90 t, and
  500 covers it; no transport force lies there; C to B is off the
  grid's axes and is charged its straight line, 212 nm (scenario 74).
  The source is base 3. The staff forms intent 4, keep base 2
  supplied, under intent 2, with a floor of 5 days, and sets its
  forces limit to source base 3 and no force. The reason carries base
  3's figures (500 t to spare, enough for the place's need of 90 t, no
  transport force there, 212 nm) and base 1 as passed over.
  `supplied_from` is base 3, `chosen_by` `staff`. From day 7 the
  supply is served each morning, says that no ship of its pool is
  afloat and sends nothing, and the hold's reason says the source was
  chosen on an earlier morning and kept; plan 2 is not marked failed
  on any day (the ships are the staff's to find: no limit of the
  commander's on the chain binds ships). Task force 1 never joins the
  pool: it never lies at base 3, and it serves intent 1. No run is
  ordered, and none would be called in the 14 days had there been a
  ship: the 90 t of day 2 leave B with 84 t on the morning of day 7
  (4 t in the landing's base and 80 t in base 2), 84 - 6 x (d - 7) on
  the morning of day d, and a run is called when that is below 42 t,
  which is day 15. Base 3 holds 500 t at the end. The same ten orders.
```

### `tests/scenario_74_the_force_assembled.rs`

```

# Restated for decision 0238

Restated by the architect on Nov 27, 2026, before the code (the staff's
choice of a source is the one rule: rulebook, chapter 8, The staff
chooses a source of supply). Where the text above and this section
differ, this section holds.

- **A unit another intent has.** The hold of base 1 (intent 2, ranked
  first, a forces limit of unit 1 alone) has men at its base and no
  source, so the staff chooses one for it on day 1, before the take is
  served. Base 2 is the enemy's. Base 3, at C, holds 1,000 t; unit 3's
  600 men eat 3 t a day there, so it keeps 15 x 3 = 45 t and can spare
  955 t; the place's need is 15 days of the 1,200 men at A, 15 x 6 =
  90 t, which it covers; task force 2 lies there with nothing aboard,
  and the hold's limit binds no ship; C is 6 hexes from A on an axis,
  240 nm. The source is base 3. The staff forms intent 3, keep base 1
  supplied, under intent 2, with a floor of 5 days, and sets its forces
  limit to source base 3 and task force 2. So the landing is intent 4,
  where the first writing had intent 3, and the limit the staff sets on
  it is as before: task forces 1 and 3, unit 2, source base 1. The
  reasons serving intent 1 say unit 1 serves intent 2, as before. No
  run is called for base 1 in the game: it holds 1,000 t against 6 t a
  day.
```

### `tests/scenario_76_controlling_an_island.rs`

```

# Restated for decision 0238

Restated by the architect on Nov 27, 2026, before the code (the staff's
choice of a source is the one rule: rulebook, chapter 8, The staff
chooses a source of supply). Where the text above and this section
differ, this section holds.

- **The words of the island's source.** The reason of the limit the
  staff sets on each hold it forms under the island carries the figures
  of the one rule in place of "N t of supply above its reserve". In the
  main game and its properties with base 1 at 1,000 t: unit 1's 600 men
  eat 3 t a day at A, so base 1 keeps 15 x 3 = 45 t and can spare
  955 t; the side uses nothing yet at B, the island's first place, so
  the place's need is 0 t; A to B is 160 nm. The words: "chosen by the
  staff from the bases of the side in no named place's hex: base:1
  (Placeholder base A): 955 t to spare, enough for the place's need of
  0 t, task_force:1 stopped there with nothing aboard, 160 nm from the
  place; the source is base:1 (Placeholder base A)". Base 3 is in a
  named place's hex and is not weighed. The choice, the limits, the
  orders and every ton are as written.
- **No source of the landing** (task force 1 held): the limits are as
  written, base 1 and no force on both holds. The reason says "the
  source is base:1 (Placeholder base A)", "no transport force stopped
  there with nothing aboard" (the first writing had "with no transport
  force ...") and "to base:1 (Placeholder base A) and no force".
```

### `tests/scenario_79_fed_before_it_is_taken.rs`

```
Restated by the architect on Nov 27, 2026, for decision 0238 (the
staff's choice of a source is the one rule: rulebook, chapter 8, The
staff chooses a source of supply, Under a commander's hold). On day 1,
after it forms intent 2, the staff sets intent 2's forces limit to
source base 1 and task force 1 and no ship: the commander's source and
his ships. That is one staff command more on day 1, a `set_limit`, and
plan 1 is amended once by it; the test that no command of the staff's
is a commander's allows this one. The reason of the forming says the
commander's forces limit names base 1 as the source. `command_state`
gives intent 1 `supplied_from` base 1, `chosen_by` `commander`. No
order, load, ton or hour below moves.

- **No source named: the staff chooses.** Restated by the architect on
  Nov 27, 2026, for decision 0238; the first writing had no supply
  formed, no order in the game and base 1 named as a base a source may
  be named among. With base 1 left out of intent 1's forces limit, the
  staff weighs the bases on day 1. Base 3 is in the place's hex and is
  not weighed. Base 1 holds 1,000 t; no man eats at A, so it keeps
  nothing and can spare 1,000 t; the place's need is 15 x 6 = 90 t,
  which it covers; task force 1 lies there with nothing aboard; it is
  160 nm from B. The source is base 1. The staff forms intent 2 and
  sets its forces limit to source base 1 and task force 1: the ships
  are the commander's, whose limit names them. The reason carries base
  1's figures. `supplied_from` is base 1, `chosen_by` `staff`. The run
  of day 3 is cut to what the source can spare, 1,000 t that morning,
  so its 90 t are not cut. The game is then the main case's, order for
  order and ton for ton: the three orders of days 3 and 4, 54 t at
  base 3, 910 t at base 1, 72 t at base 2 and 968 t in each ship at the
  end, and every man fed every night. On each morning from day 2 to day
  14 the hold has one reason more: the source is base 1, chosen on an
  earlier morning and kept. No staff order or command is refused.
```

### `tests/scenario_86_fed_with_the_lift.rs`

```

# Restated for decision 0238

Restated by the architect on Nov 27, 2026, before the code (the staff's
choice of a source is the one rule: rulebook, chapter 8, The staff
chooses a source of supply). Where the text above and this section
differ, this section holds.

- **The source of supply is base 1, named on day 1 with no force.** The
  limits of days 1 and 3 are as written. The words of day 1: nobody
  eats at A (the men are aboard), so base 1 can spare its 1,000 t; the
  side uses nothing yet at B, so the place's need is 0 t; A to B is
  160 nm: "base:1 (Placeholder base A): 1000 t to spare, enough for the
  place's need of 0 t, no transport force stopped there with nothing
  aboard, 160 nm from the place", "the source is base:1 (Placeholder
  base A)", "to base:1 (Placeholder base A) and no force". Base 4, which
  holds nothing, is not named: a base is named as passed over only
  where it holds supply and keeps it all.
- **A base with nothing cannot be the source.** With base 1 at 0 t no
  base holds supply, so none is weighed and none is named: "no base can
  be the source of supply (the bases of the side in no named place's
  hex: none)", "so the staff sets no forces limit on", "and it forms no
  supply". No limit is set on intent 3 on any day, no "keep supplied"
  is formed, and the lift's route is as written.
```
