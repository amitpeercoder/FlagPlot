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

## The card on the new game screen, cut short (Nov 27, 2026)

The designer read rung 1's card on the new game screen and found it too long and too technical for a first game: "We could do with much less, so that users may actually want to try it." The card is cut to this, in `table/scenarios.json`:

```json
"lines": [
 "Welcome, Commander. Your first command is a quiet one: one ship, one order, no enemy.",
 "Thornwick, 200 miles away, depends on you for its food.",
 "A placeholder game: everything in it is invented."
],
"sides": {
 "allied": "Keep both garrisons fed."
}
```

The entry above in this file shows the card as first built. The directive opens when the game begins and carries the mission in full, as the designer confirmed it, with every figure: its mission paragraph is unchanged.

The designer then asked that the directive be "a bit more friendly and welcoming, after all, this is the commander's onboarding mission". Three places of it changed, still in the Area Commander's voice. The situation opens "Welcome to your first command. Saltmere is yours, with its stock, its fuel and one ship, the fast transport Merrowdale." and ends "There is no enemy within reach, so take your time and learn the work." The guidance opens "You need give only one order. Tell your staff to keep Thornwick supplied: name the ship, and Saltmere as its source, and set its two numbers:" and goes on as it did. Its last paragraph ends "If it goes badly, begin again: that is how this work is learned. Good luck." The directive shown earlier in this file is the one first built. The entry's `mission`, its pairing and the scenario are unchanged. The card still has three lines, the last beginning "A placeholder game", and the header still says 14 days.

One check changed with it. `table/check_screen.py`, statement L3, held the directive's mission to be the entry's `sides.allied`, letter for letter. It now holds that `sides.allied` is "Keep both garrisons fed." and that the directive's mission begins with it: the card says the mission in brief and the directive says it in full.

## Islands on the ladder's chart, and where a rung opens (Nov 27, 2026)

The designer, of the same first look: "The map has what it needs other than proper islands. We need to make a good impression on the new player. Also, maybe have these islands more towards the middle of the map rather where it currently is, at the top-left?"

- **The islands.** `table/chart/placeholder_ladder_coast.json` is new: six invented outlines, one island under each of the map's four coast hexes, S (0,0), T (5,0), U (0,14) and E (3,3), and an islet off S and off T. Each lies inside its own hex. It is for the eye only, as every map's coast file is: the page draws it and the game reads none of it. The map, `data/map/placeholder_ladder.toml`, and the scenario are unchanged.
- **Where a rung opens.** The page has `openOnMission`: a game whose entry has a `mission` opens looking at the middle of the mission's places, leant in (zoom 0.3), where every other game opens on the whole table. Sat right back, the table is whole as before. So rung 1 opens on its two places in the middle of the view.
- **Not done: the map itself.** S and T still lie in the map's north-west, since the one map holds the far place of rungs 2 and 3 fourteen rows south. Moving them toward the middle of the table means changing the map's extent, which is data shared by three rungs, and is left for the designer's word.
- **The check.** `table/check_screen.py`, statement L3a: the coast file's note, its six closed rings, each inside the hex of one of the four places, and `openOnMission` defined once and called once.

## Rung 1's things renamed (decision 0241, Nov 27, 2026)

Under decision 0241 rung 1, a game invented whole, names its things without the word "Placeholder": base S is Saltmere, base T is Thornwick, their garrisons are the Saltmere garrison and the Thornwick garrison, and the ship is Merrowdale. Changed with it: the five names in `data/scenarios/ladder/138_one_ship.toml`; the mission's two place names in `table/scenarios.json` and the card's second line; the directive, which is to "Commander, Saltmere" and names the places and the ship; and in `table/check_screen.py` the mission, the verdict's words (W1 to W8, V1 to V3) and statement L3, all restated with the new names and nothing else. Statement L1a is new: the scenario's five names, and none of them in `data/oob`. Wherever this file above says "Base S", "Base T" or "Fast transport 1" of the game as first built, the game now says the new name.


## Rung 3 before rung 2: the ruling, and rung 3's game as planned (Nov 28, 2026)

### The ruling on the order

The manager asked whether rung 3 may be built before rung 2, since it does not wait for rung 1's play as rung 2's figures do. Ruled by the architect, in two parts.

1. **Rung 3 before rung 2: yes.** The record's steps 5 and 6 are the architect's order and no rule. Rung 3 reads nothing of rung 2's: its file names neither the far place nor the freighter, its measure is its own, and the floor and the interval it uses are rung 1's. Rung 2's figures still wait for the designer's account of rung 1 (the record, "Not yet tried widely"). The numbers stand: rung 2 is scenario 139 and rung 3 is scenario 140.
2. **Before the designer's account of rung 1: his to say, not the architect's.** The record's step 4 reads "The designer plays rung 1. His account decides whether 5 and 6 go on as planned", and "What was weighed" keeps `levels.md`'s suggestion that rung 1 is built and played first. That is his gate, and it stands before both rungs. He has played rung 1 (decision 0241 opens on his words after playing it, and this file records three of his remarks on its card, its chart and its names); none of them speaks against a rung, and none says to go on. So what is the architect's own is written now, and is at risk if his account moves rung 3: the scenario file and the hand calculation, on a branch. **No developer is briefed, nothing is merged and nothing goes on the table's list until he says to go on.**

### What is written

- `data/scenarios/ladder/140_men_then_food.toml`, placeholder data by the architect, on the ladder's map. Its things are named under decision 0241: Saltmere and Merrowdale as in rung 1; Emberholt, the empty place E; the transports Farrowby, Gadwell and Hestercombe; the Saltmere battalion. None is a name in `data/oob`.
- `tests/scenario_140_men_then_food.rs`: the hand calculation, seven cases and eleven statements, each an ignored test that fails; declared in `tests/all.rs`. On the live figures, by question 8.

**The figures settled,** beyond the record's: the source holds 200 t of supply and no fuel; each transport holds 200 t of fuel and the supply ship 300 t (a round trip to E burns 52 t in a ship), so fuel decides nothing on this rung; the unit has the standard experience, so it needs no training.

**The cases,** each checked on the engine on Nov 28, 2026:

| Case | The orders | Nights unfed at E | The place |
| --- | --- | --- | --- |
| A | Press; supply at 2 and 5 on day 13 | None; runs of 15 t on days 13 and 18 | Ours from day 12 |
| B | Press; supply on day 14 | None; runs of 18 t on day 14 and 15 t on day 19 | Ours |
| C | Press; supply on day 15 | 1 (day 16); the run lands that morning, after the meal | Ours |
| D | Press; supply on day 12 | 6 (days 16 to 21); the order is refused, the base is the enemy's | Ours |
| E | Press; a hold of base 2 on day 1, with the supply ship and the source | None; the staff forms the supply on day 13, runs of 30 t on days 14 and 20 | Ours |
| F | Balanced | None: no man is ever ashore there | The enemy's at the end; the men are aboard at S |
| G | Balanced, then press on day 17; no supply | None; landed on day 18 and fed by the rations on the nights of days 19 to 21 | Ours from day 18 |

### Three things found while writing it

1. **A ship is charged 208 nm from S to E, not 240.** The map's comment and the record count six hex steps of 40 nm. On open water a step is charged the miles it brings the force closer in a straight line (chapter 2), so the passage is 40 times the square root of 27: 208 nm, 13 hours at 16 kn. Nothing of the rung turns on it: a passage is one day either way. The map file is not changed, since its bytes are in rung 1's pins; the scenario file and the test give the 208.
2. **A hold may be given on any day up to day 13, and a supply order only on day 13 or 14.** Tried and not proved: a hold given on day 2, 12 or 13 feeds every night as case E does; one given on day 14 has the supply formed that day and the run ordered on day 15, and the men go unfed on the night of day 16. A hold at press forms the supply at a floor of 1 day and feeds every night too. Tried at press with the supply order on day 13 and on day 14, for floors of 1, 2, 3, 5, 8 and 10 days and intervals of 1, 2, 3, 4, 5, 6, 10 and 15: every pair feeds every night but the smallest (1 and 1, three nights unfed; 1 and 2, and 2 and 1, two; 1 and 3, 2 and 2, and 3 and 1, one). The source has stock to spare, so nothing fails on too much. Given on day 15, every pair has at least the night of day 16 unfed.
3. **The rung can be won with no food sent: case G.** The mission as the record has it says the place is to be ours and no man ashore there unfed "to the end of day 21", and names no day by which it is to be taken. The landing is ready at press from day 11 on, whenever press is given. Press given on day 17, 18, 19 or 20 lands the men on day 18 to 21, the place is ours that day, and the three nights of rations reach the end of the game. Tried on each of those days: mission met by its words, with no supply order and no hold. So the second half of "men, then food" can be skipped by landing late, and a player who starts at balanced, sees "ready on day 21" and presses late is rewarded for it. Press given on any day up to day 11 lands on day 12, as proved; on day 12 to 16 it lands the day after, and food is needed for the nights after the rations.

### A question for the designer

**Should rung 3's mission name the day by which the place is to be taken?** Courses:

- **(a) It names day 14.** "Seize the place, reported empty, by the end of day 14, and hold it with its men fed: it is to be ours by then, and no man ashore there is to go a night unfed, to the end of day 21. Men land with three nights of rations." Press given by day 13 meets it; the rations then end by the night of day 17 at the latest, so food must follow in every game that meets it. The verdict gains one reading: the first night the seat's own events show men of his fed or unfed in the hex is the day after the landing, so "taken by day 14" is "a report of his men ashore there on the night of day 15", read from the same events it reads already. No scenario figure, test or rule changes; case G stays as proof of what a late landing does, and the verdict calls it not met.
- **(b) It stands as it is.** Landing late is an answer he is let find, as the hold is. The rung then teaches the risk and may not teach the food.
- **(c) The game is made longer or the rations fewer.** Not offered: a longer game moves the late days and does not close them, and the rations are a figure of the ground file that every game on the table reads.

**The architect recommends (a).** The rung's name and the record's "Why it is a decision" both say the food is part of it, and a day in a mission is what a mission from above has. The game is built the same under (a) and (b); the answer is needed before rung 3 goes on the table, since it is in the mission's words and in the verdict.

### The brief for rung 3's game (class 3)

For one developer, once the designer has said to go on. Work in a worktree on a branch from `main` with the architect's commit merged; one pull request.

- **Read first:** `docs/engineering/roles-and-tests.md`; `tests/scenario_140_men_then_food.rs`, whole; `tests/scenario_138_one_ship.rs`, which is the model for every helper; `data/scenarios/ladder/140_men_then_food.toml`; this section.
- **Fill the eleven tests** of `tests/scenario_140_men_then_food.rs`, each asserting its statement exactly, and remove each `#[ignore]` as it is filled. Load as scenario 138's test does: the file's text with the case's commands added at the mark `# orders`, through `common::from_text_with` and `LoadOptions::default()`, played by `common::played::play`. Read runs from what the staff handed in, stocks and sides of bases from the truth after each day, the men fed from `MenFed`, refusals from `CommandRefused`, notices from the hand-ins.
- **The commands of each case** are in the hand calculation: both leaves on day 1; "take the place at (3,3)" as intent 1 with the forces limit of task force 1, land unit 1 and source base 1; the risk limit of intent 1; the supply order as intent 3 with a floor of 2 and an interval of 5, ship 4 and source base 1; the hold as intent 2 with the same ship and source. Scenario 67's file shows a take's commands and scenario 138's test a supply order's and a hold's.
- **Pin** the last day's hash of each game played in statement 11 in `tests/golden/pinned_hashes.txt`, as `scenario_140_a` to `scenario_140_g`, the way scenario 138's are pinned.
- **Change the doc comment's "Not built" paragraph** to "Built", in scenario 138's words. Change nothing else in the doc comment.
- **Not to be changed:** any expected number, statement or case; the scenario file; the map; any crate's `src`; any figure; any other test; `docs/rules/` and the glossary (no rule changes). Nothing under `table/` or `web/`, and no entry on the list: that is the table's pull request.
- **If a number does not come out,** stop and send the architect the case, the day, what the engine gave and the events of that day. Do not change the number, the file or the commands to pass.
- **Before the commit:** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`.
- **The architect is shown:** the diff; the output of `cargo test --test all scenario_140`; the seven pins; and `git diff --stat` against `main`, which names the test file and the pins file and nothing else.

### What rung 3 on the table needs beyond rung 1's (class 2)

Not planned in detail yet: the architect writes its section, with the entry, the directive, the verdict's words and the screen check's statements, before code, as rung 1's was, and after the designer's answer above. What it must cover:

1. **The entry,** "Rung 3: Men, then food", after rung 1's and before the Eastern Solomons, 21 days, with no date, `"engine": "absent"` and one pairing. While rung 2 is not built the list goes from rung 1 to rung 3; the entry's lines do not say a rung is missing.
2. **The measure gains a place held.** `mission` names, beside the hex whose nights count and the last day, the base to be held: base 2, Emberholt. "Held" is read from the seat's own list of its bases once the last day is fought (the record, "How a rung is judged"). Rung 1's verdict reads the seat's events alone, so this is a second reading and one more way to be not judged: the list not answered. Under course (a) it gains the day as well.
3. **The verdict's words for a place not held,** and their order with a night unfed: "Mission not met." and then "Emberholt was not ours at the end of day 21."; where it is held and a night went unfed, rung 1's sentence; "Mission met." with a second line that says both. A game with no man ever ashore there (case F) has no report of the hex at all, and that is "not held", never "met" and never "not judged". The words are settled in the section.
4. **The check that `men_fed` is read with two bases in the hex.** The engine's words name the base that paid first: "fed from base:3" on the nights of the rations, "from base:2" after. The verdict reads the hex and the unfed count and must not read the base.
5. **The directive,** in the game's own words: the situation (600 men, three transports, the supply ship, the place reported empty), the mission, and guidance that names the two things that are his, the risk and the food, without giving the days away.
6. **The page must be able to give every order of cases A to G,** and the screen check plays A, C, E and F at least: the order to capture Emberholt with the transport force, the battalion and Saltmere named and the risk set; a hold of a place that is not yet his, with the supply ship and the source; and, once the place is his, the supply order with its two numbers. The architect tries each on the page before writing the section, as rung 1's order paper was found wanting only when tried.
7. **Leave to launch.** As on rung 1 the table begins with it withheld. With the switch WHEN READY pressed on the first evening the game is the proved one. Without it the clock stops on day 11 for his leave; the architect tries that path through the seat's tools and the section says which the directive tells him, as rung 1's did.
8. **The two days must not pass unseen** (the record, rung 3, "The two bases in the hex"). With both leaves given nothing stops the clock when the place becomes his: the notice that the take is met is no standing interrupt, and the staff's first "cannot meet" does not come, since no supply order exists. A player who runs to the end from day 11 arrives at day 21 with six nights unfed. The section settles how he is told, with what the table has and no rule: the evening summary of day 12 saying the place is his, the directive telling him to stop there, or the clock run a day at a time; and the screen check holds it.
9. **The pin of offered files, the browser's check game and the pack,** as rung 1's entry moved them: `OFFERED` twelve, a plain game in `web/games/` and a game of orders for one case.
