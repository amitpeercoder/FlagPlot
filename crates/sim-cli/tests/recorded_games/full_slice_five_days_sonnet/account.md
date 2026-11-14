# The player's account

A Sonnet model, playing alone on Nov 14, 2026 with `sim-cli play` and nothing else, told to command the Allied side for the first five days of the thirty-nine of the lengthened slice (7 to 11 August 1942), and to read no file in the repository. It replaces the game of Nov 12, 2026, which was played on the fourteen-day slice and described five days that the slice no longer begins with. Its report as it gave it.

The game was played on main at the merge of pull request 297 (decision 0106), before the stock in the west was resized and the two construction units were given a standing job (decision 0101, amendments 8 and 9). Both changes are on the Japanese side, in a hex the Allied player is shown nothing of: the transcript regenerated after them gives the answers the game gives now.

Short version: I put both Marine forces ashore on day 1 and took Lunga and Tulagi that same day. By day 5 the Lunga airfield was finished. The tools were mostly enough, but I never got an air group to Lunga and I never learned how many Japanese were left there. I stopped at the day 6 briefing and did not end a sixth day.

1. UNDERSTANDING

- **Mission** (get_briefing, OBJECTIVES):
  - "Take Lunga and Tulagi, build and hold the airfield, land the convoy there, keep the field flying, and win the carrier fight." The Japanese carriers were expected from the north.
  - Scoring: ships sunk and ships crippled (carrier 10/5, battleship 8/4, cruiser 4/2, destroyer 1/0.5, transport 2/1, auxiliary 1/0.5), 0.05 per enemy aircraft, 0.005 per ton unloaded at Lunga, 10 for Lunga "open" each day, 10 for holding Lunga, 5 for holding Tulagi, and 1 per hundred enemy men lost.
  - Victory margins: 3 minor, 8 major, 16 decisive.
  - The game is 39 days long, but I only played 5.
- **Forces** (get_briefing, list_forces, get_entity):
  - Two carrier task forces (TF16 with Enterprise, TF11 with Saratoga) with fighter, dive-bomber and torpedo groups.
  - Two big transport task forces, TF9 for Guadalcanal and TF10 for Tulagi, with about 11,000 and 7,000 men and about 3,270 t and 1,822 t of supply.
  - Several smaller transport groups at Espiritu Santo.
  - Seven land groups on the Marine side: Groups A and B, the Support Group, the Raiders, Parachutists, 2/5 and the 2d Marines.
  - Three working units: the Pioneer Battalion, the CUB One detachment and the Seabees.
  - Some shore-based air groups listed at Lunga with "0 ready, 0 pilots".
- **Bases** (list_bases): I owned only Espiritu Santo at the start. Lunga and Tulagi appeared as enemy bases.
- **Landing** (the issue_orders description): prepare_landing, a route into the hex, and unload.
- **Fighting ashore and work** (the same description): set_posture attack or defend, march, and set_activity work with a job. These descriptions, not any tutorial, were how I learned the mechanics.

2. DAY BY DAY

**Day 1**
- Orders:
  - TF9 got prepare_landing at (2,-4) naming land units 1 to 4, a route to (2,-4), and unload. TF10 got the same for (3,-5) with units 5 to 8.
  - Both carrier groups moved to (4,-3), about 100 miles from Lunga.
  - Fighter groups 8 and 12 went on combat air patrol.
  - Dive-bomber groups 10 and 14 went on search, with arcs over the north and east, 350 miles.
  - Dive-bomber and torpedo groups 9, 13, 11, 15, 16 and 17 went on naval strike.
- All 20 orders were accepted.
- Result: both convoys landed in the first night and morning, in batches of about 80 or 240 men per ship per pulse. Fights ashore happened at both places.
  - At Lunga my units fired but I saw no result. I lost about 20 men in total.
  - At Tulagi 420 enemy men surrendered.
  - Both bases flipped to me: "Base taken yesterday: base:1 Lunga ... is yours now". Lunga had 540 t of supply and 200 t of fuel, and 540 t of supply and 200 t of fuel were also lost in the change.
  - Landing also created two small "Landing at (x,y)" bases, base:8 and base:9.
  - Loss line: "Aircraft lost: VF-6: 3 written off [event 168]", plus one each from VS-5 and VS-3. I never found out why.
  - The briefing estimated about 1,219 enemy men still ashore at Lunga.

**Day 2**
- Orders:
  - TF15 (Seabees, with a landing prepared at Lunga with land unit 10), TF12 (2,000 t of supply) and TF11 (CUB One, 120 t) sailed from Espiritu Santo to Lunga. Each had a prepare_landing where it carried troops, and an unload.
  - TF3 got load supply.
  - The Support Group (land unit 3) was set to work on the airfield.
  - Land units 5, 6 and 7 were set to defend.
- All 13 orders were accepted.
- Result: the fight at Lunga continued, and the briefing's enemy estimate rose to about 1,595. My Lunga units went to fatigue 80 and dropped back to defend ("attacked and tired").

**Day 3**
- Orders: attack at Lunga again with units 1, 2 and 3 (when_spent hold), and land unit 8 to defend. All 4 accepted.
- Result: fatigue went to 100 on all three units, and the enemy estimate stayed at about 1,580. I could not tell whether the attack accomplished anything. Airfield work was at zero progress on days 2 and 3.

**Day 4**
- Orders:
  - Units 1, 2 and 3 to defend and rest.
  - Empty transport task forces 9, 10 and 11 sent home to Espiritu Santo.
  - TF3 (960 t of supply) sent to Lunga with an unload.
- All 8 accepted.
- Result: the Seabees arrived on schedule. The Pioneers, CUB One and Seabees worked on the airfield without any fighting, and the airfield rose from 13,763 to 15,594 of 20,000.

**Day 5**
- Orders: units 1 and 2 set to work on the airfield, and TF12 and TF15 sent home. All 4 accepted.
- Result at the day 6 briefing: "Works: an airfield". The work log shows 21,851 of 20,000 done.
  - Fatigue showed Group A and B at 90.
  - TF3 was at (5,-4), still on its way to Lunga.

Nothing was ever refused. I found no air contacts, no ship contacts and no sea fights in 5 days.

3. WHAT I COULD NOT WORK OUT

- **Enemy men at Lunga:** the estimate was stuck at about 1,580, reported on day 3, and it was never refreshed. I could not tell whether my attacks killed anyone. I only saw "fired ... with 1719 men, 2.775972 shots". That did not stop me, but I could not judge whether to keep attacking or rest.
- **Fatigue:** it is shown (60, 80, 100) but not explained in the briefing. A unit gaining 20 a day while attacking and losing 5 a day while resting is only discoverable by watching. Attacking three days running drove my units to 100, which I did not foresee. It did not obviously stop me, but I wasted a day on a doomed attack. Resting is very slow.
- **Why the airfield did not progress on days 2 and 3, then did:** I guessed a contested hex blocked it. I never confirmed it. It stopped me for roughly two days.
- **Why the air groups at Lunga show "0 ready, 0 pilots":** the groups listed at Lunga (1 to 7) are not at the field. I did not know whether the field finishing would bring them in or whether I had to rebase, so I never rebased carrier groups to Lunga. The rebase text says it is refused if there is no airfield, and "no air group can rebase to it". I stopped before testing it after the field was finished. I could not work out when any aircraft would ever fly from Lunga, and I could not see whether the "Lunga open" objective was being earned (the briefing says "nothing you can see shows it").
- **The three-pulse fight:** I expected the landing night to be dangerous. It was not, because it went smoothly.
- **Dry bases:** the repeated "No fuel: base:1 Lunga has no fuel and task_force:9 stopped there" warnings looked alarming. I never worked out whether they mattered for transports with no mission needing fuel.
- **Odd base list:** list_bases only gave Espiritu Santo as mine at the start and listed Lunga as "held by japanese". After the landing the briefing showed Lunga and the new "Landing at (2,-4)" bases both mine. Two bases in one hex was confusing. The supply total reported for the hex joined the two.
- **When to send transports home:** I sent the empty ones home on day 4, mostly by instinct. Nothing said when they should leave. They were still at Lunga on day 3 with empty holds.
- **Ships' supply and personnel splits:** the "Landed yesterday" lists are very long, with 150 or more lines, and made the briefing hard to read.

4. LANDING AND FIGHTING ASHORE

- **Landing:** I understood it from the tool text: prepare_landing with a hex and a roster, route to the hex, and unload. It worked on the first try. I did not use rehearsal or loading preparation. The briefing said my landing was "at 0.5 of the port's rate". It still finished within a day for the large convoys.
- **What men do once ashore:** they fight automatically if an enemy is in the hex and their posture is attack. They were set to attack by default. They land in several pulses, and the first fights happened on the day of landing.
- **Making them attack or defend:** set_posture. Tired units are returned to defend automatically ("attack spent"). I used when_spent hold on day 3. I could not see the effect on the enemy.
- **Construction and labour units:** the Pioneers, CUB One and the Seabees are "working" units, and I set the work job to airfield. They never fire. The work text says their output is men times fed share times a rate times skill. I understood the formula. I did not learn how many men the field needed. The jump on day 5 came from setting two Marine groups to work.
- **What an airfield needed:** the tool text says 20,000 work, and I learned the daily rates by watching (Pioneers 659, CUB 123, Seabees 364, Support Group about 685, Marines about 2,100 to 2,300 each). Nothing in the tools said what a flying airfield additionally needs beyond supply and a ready aircraft, apart from the objective line "open at the end of each day (supply and an aircraft ready)".
- **When transports should leave:** never stated. I sent them home when the cargo and men were off.

5. RESULT AFTER DAY 5

- **What I hold:**
  - Lunga and Tulagi, which are mine from day 1.
  - A finished airfield at Lunga (the work log shows it done).
  - Lunga stocks of about 3,600 t of supply (from the day 2 and 3 briefings), with TF3's 960 t arriving.
  - Tulagi stocks of about 1,660 t.
  - Both carrier groups intact at (4,-3), with nothing sighted.
- **What I lost:** about 25 men at Lunga on day 1 and about 60 at Tulagi, a few written-off aircraft (3 from VF-6, 1 from VS-5, 1 from VS-3, 1 from VF-5), and no ships.
- **Enemy:** I believe about 1,580 Japanese are still in the Lunga hex, with low confidence (it is a stale estimate, "wrong by up to half"). I know nothing about Japanese ships or carriers. My searches and patrols found nothing in 5 days, but I do not know how good that search coverage was.
- **Score:** I cannot see my score. Several objectives are hidden until the end.

6. THREE THINGS THAT WOULD HAVE HELPED

1. A clear statement, after the airfield is built, of what to do to put aircraft on it, and why the Lunga air groups show 0 ready (is it arrivals, or rebase?).
2. A short fatigue and rest rule in the briefing or tool text (gain per attack day, recovery per day, effect on fire), and some feedback on the enemy's losses at Lunga so I could decide whether to keep attacking.
3. A shorter briefing: the per-ship landing lines should be summarised, and there should be one line saying whether Lunga is "open" today.

CALL COUNT AND FILES

- The game file holds 32 recorded calls (play-ending calls plus the queries and orders).
- In total I ran about 50 `sim-cli play` invocations. About 5 of those were not recorded: the bare usage print, 2 or 3 `--tools` calls and some of the calls that failed on the shell side.
- I read no repository file. I did save the output of `play` into scratch files in `/tmp/flagplot-game/` (`tools.txt` and `b2.txt` to `b6.txt`) and then read slices of those with grep and sed, to avoid the output limit. That was only play output.
- I also once used the harness's read of an automatically saved copy of one play answer, which is also play output.
- Game file kept at `/tmp/flagplot-game/game.jsonl`.