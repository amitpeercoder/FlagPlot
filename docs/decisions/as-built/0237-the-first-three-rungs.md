# 0237, as built: The first three rungs

## Planned: rung 1 on the table (Nov 27, 2026)

Written by the architect before any code, as the record's class table says: what the pull request "rung 1 on the table" (class 2, the record's step 3, second half) makes, in words and in values. It is built from `main` once rung 1's game (scenario 138) and the chart of a hand-written map are merged. The statements of its screen check are the doc comments of four functions in `table/check_screen.py`, written with this section; each fails "not built" and none is called from `main()`. The developer fills them and adds the calls. A statement or an expected value that the build cannot meet goes back to the architect; it is not changed to pass.

### Two things found while writing this

1. **The table cannot give rung 1's order today.** The record lists "the supply order with its floor, interval, forces and source" as built and used as it is. That is true of the engine and of the seat's tools, and false of the page: its order paper offers "Keep it supplied, 30 days" and nothing else (`table/index.html`, `ends`), and the page never sends `interval_days`. A floor of 30 days with no interval sends 30 t twice and the source goes unfed, so the rung cannot be won from the page. The designer's first answer makes the floor and the interval his, so the paper learns the two numbers in this pull request ("The order paper", below). It is what the table gives, with the rules as they are: class 2, and no other game's order moves.
2. **Leave to launch is withheld by default, and the hand calculation gives it.** Every case of `tests/scenario_138_one_ship.rs` gives both leaves on day 1. On the table a game begins with leave to prepare given and leave to launch withheld, so the staff readies its plan, the clock stops on "the plan waits for the commander's leave", and he approves it. Tried by the architect on Nov 27, 2026 through the seat's tools (the order previewed and approved, the plan approved at the stop, `run` to the end): 2 and 5 is met and 2 and 6 fails at the source as proved, but the band is not the proved one. 2 and 4 is met, and a floor of 1 day fails by one night at the forward place whatever the interval. A trial is not a hand calculation. The designer settled it ("The designer's answer", below): he gives leave to launch himself on the first evening, by the switch the page has, and the game he plays is the proved one.

### What the player meets

He starts the table, and the new game screen lists "Rung 1: One ship" first. He picks it and reads three lines and the mission. The Allied seat is the only one offered. He presses BEGIN. The directive opens of itself, as every game's first directive does: the challenge. He touches Base T, picks "Keep it supplied", names the ship and Base S, sets two numbers and signs. Under YOUR ORDERS he presses WHEN READY, so that plans launch when ready, and carries on. The clock stops when the staff cannot meet the order, which with a right answer is day 13. When day 14 is fought, "Read how it ended" says whether the mission was met.

### The entry

First in `table/scenarios.json`, before the Eastern Solomons. Every other entry is byte for byte what it was.

```json
{
 "id": "ladder-1-one-ship",
 "map": "data/map/placeholder_ladder.toml",
 "name": "Rung 1: One ship",
 "dates": "An invented sea",
 "days": 14,
 "placeholder": true,
 "lines": [
  "The first rung of the ladder: one ship, one order, no enemy.",
  "You command at Base S, which has the stock, the fuel and the ship. Base T, 200 miles away, is the forward garrison.",
  "A placeholder game: the sea, the bases, the ship and the men are invented."
 ],
 "sides": {
  "allied": "Keep both garrisons fed. Every man, here and forward, is to be fed every night to the end of day 14. The forward garrison holds five days of food. Our own men eat from the stock you ship from, and it holds 5 t more than both need. Your one ship carries six days of the forward garrison's food at a time, and the fuel on hand is enough for two round trips."
 },
 "islands": [],
 "directives": {
  "allied": "table/directives/ladder_1_allied.json"
 },
 "mission": {
  "measure": "every_night_fed",
  "last_day": 14,
  "places": [
   { "q": 0, "r": 0, "name": "Base S" },
   { "q": 5, "r": 0, "name": "Base T" }
  ]
 },
 "pairings": [
  {
   "allied": "person",
   "japanese": "engine",
   "engine": "absent",
   "file": "data/scenarios/ladder/138_one_ship.toml"
  }
 ]
}
```

- **The mission's words** are the record's, as the designer confirmed them, letter for letter. The lines name the two places, because the verdict names them.
- **No `start`.** The page counts days: "Day 1", "DAY 3 OF 14". The screen's small line reads "An invented sea · 14 days".
- **No `japanese` under `sides` or `directives`,** and one pairing: no agent, no game from the other side.
- **`"engine": "absent"`** is a new value. The screen says for it: "There is no enemy in this game: the Japanese side has nothing." (`ENGINE_IS`, one line added; the three it has are unchanged.)
- **The measure is data.** `every_night_fed` is the only measure there is. `places` are the hexes whose nights count, in the order the verdict speaks of them, each with the name the verdict uses, which is the base's name as the page shows it (`cleanName` of the seat's own name for its base in that hex; statement L6 holds the two equal). `last_day` is the last day whose night counts.
- **The note** at the head of the file gains one sentence, after "Each pairing names who holds each side (person, agent or engine) and the file.": "A rung of the ladder (decision 0237) is an invented game with no enemy: its other side is the engine's and has nothing (\"engine\": \"absent\"), it names no date for day 1, and its \"mission\" is the measure its verdict is read by."

The bridge gives the game's page the mission as `config.mission`, the entry's value unchanged, and gives no such key to a game whose entry has none (`table/serve.py`, `open_game`; `web/door.js`, where it builds the same config). What the screen is given of a scenario (`offered`) is unchanged.

### The directive

`table/directives/ladder_1_allied.json`:

```json
{
 "note": "The directive for the Allied seat of rung 1 of the ladder (decision 0237). A placeholder: the command, the places, the ship and the men are invented, and the page shows each name without the word. Written for the game in its own words. The mission is the record's, as the designer confirmed it.",
 "directives": [
  {
   "number": 1,
   "when": "start",
   "from": "Area Commander",
   "to": "Commander, Base S",
   "subject": "Both garrisons fed",
   "parts": [
    {
     "heading": "1. Situation",
     "text": [
      "You command at Base S. It has the stock, the fuel and one ship, Fast transport 1. Base T, 200 miles away, is the forward garrison. Each place has 1,000 men, who eat 5 t a day. There is no enemy within reach."
     ]
    },
    {
     "heading": "2. Mission",
     "text": [
      "Keep both garrisons fed. Every man, here and forward, is to be fed every night to the end of day 14. The forward garrison holds five days of food. Our own men eat from the stock you ship from, and it holds 5 t more than both need. Your one ship carries six days of the forward garrison's food at a time, and the fuel on hand is enough for two round trips."
     ]
    },
    {
     "heading": "3. Guidance",
     "text": [
      "Give the staff one order: keep Base T supplied. Name the ship, and Base S as its source, and set its two numbers: the days of food Base T is never to fall below, and the days more that each run brings. The staff does the rest: it sizes each run from those two numbers, loads the ship and sails her.",
      "Then tell the staff that plans launch when ready, under YOUR ORDERS, and carry on: it will not sail the ship without that leave.",
      "When day 14 is fought you are told whether the mission was met and, if it was not, which garrison went unfed and on which nights."
     ]
    }
   ]
  }
 ]
}
```

The second paragraph of the guidance is the designer's answer ("The designer's answer", below).

### The order paper: the floor and the interval

Where the end chosen is "keep it supplied", the paper shows one more row, as it shows the patrol's distance for a patrol (`o-patrol`):

- The row `o-supply`, hidden for every other end, with two whole numbers. `o-floor`: "Never below" [n] "days of supply", from 1 to 99, starting at 30. `o-interval`: "Each run brings" [n] "days more", from 1 to 99, starting empty; while it is empty the row says "left to the staff: 15 days".
- The end reads "Keep it supplied", with no number in its name.
- **What is sent.** `floor_days` is the number shown: 30 where the field is empty or no number, and held to 1 to 99. `interval_days` is sent where the field holds a number, held to 1 to 99, and is not sent where it is empty. So a paper nobody touched sends what it sends today, key for key: `intent_kind`, `base`, `floor_days` 30.
- A change of either number asks the staff's reading again, as a change of the patrol's distance does.
- A place's card says an order in force with its numbers: "Kept supplied, 2 days, each run 5 days more"; with no interval set it says what it says today.
- Nothing else of the paper changes: the ship and the source are named under LIMITS as today, and no word for any other end moves.

### The verdict

**What it reads.** The seat's `events` tool, once for each day from 1 to `last_day`, through the page's `dayEvents`, and `config.mission`. Nothing else: no briefing, no list of bases, no result, no replay, save or log, and nothing the page kept from earlier in the game. A page loaded again after the end asks again.

**When.** Once, when the game is over, on the paper HOW IT ENDED, which "Read how it ended" opens. Nothing is said of the mission during play beyond what the page says today: the directive stays under Staff, and each evening's summary from the staff says of a night unfed how many men went unfed and where. A game lost on day 7 is told as lost on day 14. The night's meal is the first thing of a day, so the last night is among the events of day `last_day`, and it is read after that day is fought.

**What counts.** An event of kind `men_fed` in the night pulse whose words give a hex of `places` and an unfed count above 0. The words are the engine's: "Allied men ashore at (5,0): 0 of 1000 fed from base:2, 1000 unfed", or, where the side has no base in the hex, "Allied men ashore at (3,3): no base of the side, none of 600 fed, 600 unfed". Two such events of one hex in one night are added. A `men_fed` event whose words fit neither form, or a day that was not answered, makes the mission not judged: the page never says "met" of a game it could not read whole.

**The words.** The page's own function `missionLines(mission, days)`, which draws nothing and asks nothing: `days` holds each day's events, day 1 first, or nothing for a day not answered. It gives the lines, the first of which is one of three.

- **Met:** "Mission met." and then "Every man was fed every night to the end of day 14, at Base S and at Base T." The places are the measure's, in its order, each named: one place is "at A.", two each take their own "at", "at A and at B.", and three or more are joined as English joins a list under one "at", "at A, B and C."
- **Not met:** "Mission not met." and then one sentence for each place that had a night unfed, in the measure's order, and within a place one sentence for each count of men, in the order of its first night:
  - "1,000 men at Base T went unfed on the night of day 14."
  - "1,000 men at Base S went unfed on the nights of days 13 and 14."
  - "1,000 men at Base T went unfed on the nights of days 7, 8, 9, 12, 13 and 14."
  - A count of one is "1 man at". Numbers take their commas.
- **Not judged:** "The mission could not be judged." and then "Day 9 could not be read." for the first day not answered, or "A report of the men fed on day 9 could not be read." for the first event that did not fit.

**On the paper.** Where the game has a mission, HOW IT ENDED opens with the verdict in place of "The game is over. This game set no objectives, so the umpire gives no result.": the first line as a stamp, "MISSION MET" as a victory is stamped and "MISSION NOT MET" as a defeat is, the other lines under it, then "Day 14 of 14 is fought." and the button for the war diary as today. Not judged has no stamp: its two lines are plain. While the days are being asked the paper says "Reading the nights." Nothing is scored, nothing is compared with the staff, and a game with no mission reads as it does today.

### The screen check

Four functions in `table/check_screen.py`, above `main()`, each with its statements in its doc comment:

| Function | Holds |
| --- | --- |
| `rung_one_on_the_list(bridge)` | L1 to L9: the entry, the directive, the screen, the game begun with no date, and the switch for leave to launch |
| `the_supply_orders_two_numbers(bridge)` | O1 to O6: the row, what is sent, and that an untouched paper sends what it sent |
| `the_verdict_in_words()` | W1 to W9: `missionLines` in node on events of the tool's shape |
| `rung_one_played_three_ways(bridge)` | V1 to V4: 2 and 5, 2 and 6, 2 and 4, each with leave to launch given; what the verdict read |

The expected verdicts of V1 to V3 are the nights of statements 3, 5 and 6 of `tests/scenario_138_one_ship.rs` (cases A, B and C), said in the verdict's words:

| Game | The lines |
| --- | --- |
| 2 and 5 | "Mission met." / "Every man was fed every night to the end of day 14, at Base S and at Base T." |
| 2 and 6 | "Mission not met." / "1,000 men at Base S went unfed on the nights of days 13 and 14." |
| 2 and 4 | "Mission not met." / "1,000 men at Base T went unfed on the night of day 14." |

**Three expectations of the checks that stand today move, on the architect's word, and no other:** in `the_screen`, the list of ids gains `ladder-1-one-ship` first and the counts of games gain 1 first; and the loop that asks every side of every entry for a directive at the start with a mission asks it of each side that a person or an agent may hold in some pairing of the entry, since a side only the engine ever holds needs none. `PAGE_LOAD_CALLS` is unchanged: the verdict's reading is at the end and is no part of a page's load.

### The pin of offered files, the browser's check and the pack

- **`crates/sim-cli/tests/pack_from_source.rs`.** `OFFERED` becomes eleven, with `data/scenarios/ladder/138_one_ship.toml` added last, so that `OFFERED[4]`, the file two tests take apart, is still Midway's. The two comments that say "ten" say "eleven". Nothing else in the file changes, and its three tests pass.
- **`web/games/138_one_ship.mjs`:** the plain game, as the ten others are: the file, the seat `allied`, `toTheEnd`. It is statement 10's, whose words in the head of `web/check.mjs` become "Each of the eleven offered files".
- **`web/games/orders-ladder-one-ship.mjs`,** and a statement 17 in the head of `web/check.mjs`, copied word for word, with its number in the table of statements by name: "17. Rung 1, the Allied seat: the reads of a page's load, then the supply order for base 2 at a floor of 2 days and an interval of 5, with ship 1 and base 1 as its forces, previewed and then signed, leave to launch given, then `run` to the end with whatever stops it answered by `run` again, then `events` of each day from 1 to 14 as the verdict asks them. Every answer and the saved game are the native door's, byte for byte." No expected value is written in the game: the check compares the two builds.
- **The pack.** `node web/offered.mjs` prints eleven files, the rung's first. `sh scripts/web/build.sh` packs them, makes the chart of the ladder's map for the site and copies the directive; no line of the script changes. `web/door.js` gives the page `mission` as the bridge does.
- **The equality the architect is shown:** `node web/check.mjs` ends "Every check held", with the lines of statement 17, of statement 10 for `138_one_ship`, and "web/games holds one game of the check for each of the 11 scenario files the list offers, and no other plain one".

### What must not change

- No crate's `src`, no `Cargo.toml`, nothing under `data/`, nothing under `tests/`, no rule, nothing in `docs/rules/` or the glossary. Scenario 138, its map and its test are as merged. Of `crates/`, the one list in `pack_from_source.rs` and its two comments.
- Every other entry of `table/scenarios.json` and every other directive, byte for byte.
- What the page sends for an order to keep a place supplied that nobody touched, and every word the page says for any other end.
- `PAGE_LOAD_CALLS`, `theReadsOfAPageLoad`, and every statement of `web/check.mjs` but the count in statement 10 and the new statement 17.
- Every expectation of `check_screen.py` and `check_watch.py` but the three named above.
- No agent's pairing, no dry run (the seat of this file is not offered `trial`, and the paper shows none), no points, no date.
- `table/README.md` is amended to say what is new: the entry and its keys `mission` and `engine` "absent", a game with no date, the two numbers on the order paper, and the verdict on HOW IT ENDED.

### What the architect is shown

The pull request's diff, and the output of: `python3 table/check_screen.py` (the lines of the four functions, and "all held"); `node web/check.mjs`; `cargo test -p sim-cli --test pack_from_source`; `node web/offered.mjs`; `git diff --stat` against `main`; and the paper HOW IT ENDED of the three games of V1 to V3, as text or as a picture.

### How to start it

From the top of the repository:

```
cargo build --release -p sim-cli
python3 table/serve.py
```

The bridge prints "The table is at" and an address. Opened in a browser, it shows A NEW GAME with "Rung 1: One ship" first on the list: pick it, and BEGIN. The browser's build has it too: `sh scripts/web/build.sh`, then `python3 -m http.server -d target/web/site 8000`.

### The designer's answer (Nov 27, 2026)

**The question put.** On the table every game begins with leave to launch withheld: the staff readies its plan, stops the clock and asks. The answers proved by hand give that leave on day 1. Three courses were offered: (a) he approves the plan at the stop, the directive says so and the architect adds a case H to the hand calculation for that path; (b) the scenario gives leave to launch itself, a class 3 change to scenario 138; (c) the directive tells him to give the leave himself.

**His question back:** "Since the game is now about planning - can the plan be approved before the clock starts running?"

**The reading, from the code and from runs of the engine through the seat's tools that day.**

- The plan exists on the first evening. Once the supply order is signed at 1800, before day 1 is fought, the seat's `plans` lists plan 1 as a draft with no run yet.
- Approving that plan then does not launch it. `approve` of a plan prepares a draft or releases a plan that is preparing; on the first evening it moves plan 1 from draft to preparing, and the clock still stops later on "the plan waits for the commander's leave".
- The switch does. Under YOUR ORDERS on the stop paper the page has the line "Plans launch: on my word" with the button WHEN READY, which sends `set_leave` for the leave to launch. It can be pressed on the first evening. With the order signed and the switch pressed before day 1, the game is the hand calculation's: 2 and 5 met, 2 and 6 the source unfed on the nights of days 13 and 14, 2 and 4 the forward garrison unfed on the night of day 14, and the whole table of the record's "Rung 1 with two sides to fail on", cell for cell.

**Settled: course (c), by the page's own switch, before the clock runs.** No scenario changes and the page needs nothing new for it. The directive's guidance says it in its second paragraph, as written above. V1 to V3 stand as written: they give leave to launch with the order. V5 (the plan approved at the stop) and case H are withdrawn: that path is play, and is not proved or checked. The list check gains L9: the switch is on the page and can be pressed before day 1.
