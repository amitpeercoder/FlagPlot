# 0072: The Cape Esperance scenario: what is in it and what is left out

Status: built and run once (Nov 13, 2026); settled by the architect, to be reviewed. Drafted by the architect's team lead on Nov 13, 2026. Second stage built the same day: the ships derived and the scenario written, not run ("Built", below). Third stage the same day: the scenario moved to `data/scenarios/` and the check run once, judged 8, agree 4, at the edge 1, miss 3 ("As built (the check)", at the end). The data choices below are settled as the architect's (decision 0015 and `docs/engineering/data-rules.md`, "Who settles data") and wait for the architect's review. No rule is added or changed.

**No batch of this scenario is run or looked at before Savo Island is tuned, and it is never tuned on.** This stage ran no batch, no `validate` and no run of the scenario, and nothing below was set in answer to an outcome.

## The gap

Step 2 tunes the surface figures on Savo Island and checks them on Cape Esperance, the night of 11 to 12 October 1942, where the other side had the radar and the surprise (`docs/step2-spec.md`, "Tuning"). The spec says Cape Esperance is built after Savo Island is tuned and needs its ships derived and a scenario. This record prepares what does not need tuned figures: who is there, where and when they meet, how ready each side is, what the rules can and cannot show of the night, and the ships' class figures as far as the facts allow. Its form follows decision 0024, which did the same for Savo Island.

The facts are in `data/oob` under the engagement `cape-esperance` and are described in `docs/research/oob/cape-esperance.md`. The measures the scenario will be judged on are `data/validation/cape_esperance.toml`, whose scope this plan keeps.

## Decisions

### Who is there

1. **Two forces fight: Task Force 64 and the bombardment group.** The scope is the night action only, as `data/validation/cape_esperance.toml` assumes.
   - **Task Force 64 (Scott), nine ships,** in the battle disposition of 22:35 (engagement_positions 522): destroyers Farenholt, Duncan, Laffey; cruisers San Francisco, Boise, Salt Lake City, Helena; destroyers Buchanan, McCalla.
   - **The bombardment group (Goto), five ships:** Aoba, Furutaka, Kinugasa in column, Fubuki to starboard of Aoba and Hatsuyuki to port (engagement_positions 506).
2. **The reinforcement group is left out:** seaplane tenders Nisshin and Chitose and six destroyers. It passed Savo three hours before Goto, unloaded at Tassafaronga from about 22:10 and took no part in the action (engagement_positions 519; the research report). Tassafaronga is in the hex of the battle, (2,-5), and Task Force 64 enters that hex at 22:20 (below). The rules have no way for a force to pass an enemy it shares a hex with (decision 0024, 2), so with the group in, the American force would fight it on arrival, which did not happen: San Francisco's floatplane reported it at 22:50 and Scott did not attack it. Left out for the same reason as Savo Island's pickets. Murakumo and Natsugumo, sunk by aircraft the next day while helping survivors, go with it; their rows are kept apart in the facts.
3. **Each side is one force.** The American column's turn at 23:32 to 23:36, and the Japanese column's turn away after the first hits, are inside the hex and are not modelled (as at Savo Island, decision 0024, 3).

### Where and when

The map puts every point of the action in hex (2,-5), the hex of Savo Island and of Savo Island's scenario: the contact at Savo bearing 280, 15 miles (engagement_positions 501, 502), 5.6 miles from the hex's centre; the battle position and Fubuki's sinking position (534, 536); Furutaka's sinking, Boise's 00:55 position and Duncan's (503, 538, 541); Cape Esperance and Tassafaronga (`place_positions.csv` 32, 51). Worked by the projection of decision 0017 (`crates/map-tool/src/projection.rs`). Ships are charged straight-line miles (decision 0058), from `map-tool path` on the real map.

4. **The game begins at 18:00 on 11 October, zone minus 11,** the start of the night pulse. It runs one day. The American sources keep zone minus 11; the Japanese record keeps a clock two hours earlier (its 21:46 is the American 23:46), and its times are moved two hours on here.
5. **Task Force 64 starts at 18:00 in hex (2,-2), steams for (1,-4) and then (2,-5) at 29 knots, and stays in (2,-5).**
   - No source gives the 18:00 position. The force left 11°30'S 161°45'E at 16:00 (engagement_positions 510, 511; hex (3,-1)) and Salt Lake City fixed her position at 10°42'S 160°14'E at about 18:45 (513; hex (1,-2)). The point 2/2.75 of the way between them, 18:00 on a straight run, is 10.92°S 160.65°E, in hex (2,-2), 4.8 miles from its centre. The two rows are 101 miles apart in 2 3/4 hours, faster than the stated 29 knots, so one of them is loose; neither is preferred over the other, and the start is between them.
   - The route goes by (1,-4), the hex of the force's turn north at 9°43'S 159°26'E (515), because the straight path from (2,-2) to (2,-5) runs east of Guadalcanal, through (3,-3), (3,-4) and (3,-5), and the force went round the western end.
   - 29 knots is the speed of the approach (510, 511: high, preferred). The force slowed to 25 and then 20 knots after 21:15 (515, 516, both disputed). One speed is ordered for the day; 29 is the stated one, and it fits the times: the force reaches (1,-4)'s centre at 21:39 (the sources give 21:15 and 21:45 for the turn north), enters (2,-5) at 22:20 (it rounded the tip of Guadalcanal just before 22:30, 520) and reaches the centre at 23:02 (at 23:00 Savo was dead ahead, six miles, 524).
   - It stays in (2,-5): Scott's plan was to patrol off Savo and Cape Esperance, and the route ends there. Its turn away to the south-west after 00:27 (537) is not ordered: a route out would put the force under way when the Japanese arrive, which decision 0022, 6 handles only approximately.
6. **The bombardment group starts at 18:00 in hex (0,-8), steams for (2,-5) and then Shortland, hex (-2,-8), at 27 knots, and enters (2,-5) at 23:42.**
   - At 18:26 the group was at Guadalcanal bearing 310, 200 miles (engagement_positions 494, medium, preferred), and from 18:00 it made 30 knots (495). "Guadalcanal" is read as Lunga, the reference point of the American air reports of the same day; the point is in hex (0,-8), 12 miles from its centre. Read from Savo instead, it is 187 miles from the contact, which 30 knots does not cover in the time; from Lunga it is 167 miles, which 30 knots covers with the time the squalls took (502).
   - From (0,-8) the path is (1,-8), (1,-7), (2,-7), (2,-6), (2,-5), charged 30, 38, 26, 40 and 40 miles, 174 in all, clear of land. The group enters (2,-5) halfway through the last step, after 154 miles.
   - **The speed is worked back from the hour, as at Savo Island (decision 0024, 6).** Fire opened at 23:46 in every source (engagement_events 510 and the US rows). At 27 knots the group enters (2,-5) after 154 / 27 = 5.703703 hours, at 23:42, four minutes before. 27 knots lies inside what the sources give for the approach: 30 by the Japanese report, 26 by one history and by the report's own reduction for the bombardment at 23:43, 20 by the American radar plots (495, 497, 517, 518, 527). At the report's 30 knots from this start the group would enter at 23:08, 38 minutes early; from (-1,-8) it would enter at 00:16.
   - Its route goes on to Shortland, where it came from and where Aoba and Kinugasa returned (engagement_positions 493; outcomes 1508). The bombardment of Henderson Field it was sent for is not modelled: shore bombardment is left out of step 1 (decision 0052), so the route does not go on to Lunga.
7. **The meeting is in the night pulse, in hex (2,-5), at 23:42 (5.703703 hours after the start), when the bombardment group enters the hex where Task Force 64 has stood since 23:02.** It is dark. No other meeting happens: at 22:20, when Task Force 64 enters (2,-5), the Japanese are in (2,-6), which they entered at 22:13 (after 114 miles, 4.222222 hours), a different hex; and no force is in a hex with an enemy at the start of any pulse.

### Readiness and what each side knows

8. **Task Force 64 is at battle readiness from the start.** It set Condition One at 18:15 (engagement_events 584, 585: high, preferred), fifteen minutes after the scenario begins, and expected contact about 23:00. A readiness order takes effect for a whole pulse, and nothing happens in the first fifteen minutes. So it is ready and alert at the meeting (chapter 4, Ready, and the share of the guns).
9. **The bombardment group is at normal readiness and not alert.** Its staff judged before sailing that a counterattack by a large surface force had almost no chance, and no enemy report had come from its searches or from the reinforcement group (engagement_events 497, 718, 719). It advanced in its No. 9 alert cruising disposition, not at battle stations (720), and was not at general quarters at 23:43 (544, low, unconfirmed: one secondary source, consistent with the rest). It took the ships it saw for its own landing group (507, 508, 722). Goto ordered battle at 23:43 (509), and fire came at 23:46 as Aoba was about to open (510, 723); Furutaka ordered battle on seeing the flashes at 23:46 (724), her high-angle guns opened at 23:48 and her main battery at 23:52 (727, 731), Kinugasa at 23:52 or 23:54 (725, 732). The ships' rounds were Aoba 7, Furutaka 20 and Kinugasa 80 main-battery rounds (engagement_outcomes 2276, 2278, 2280; Fubuki unknown, 2283). All times here are moved two hours on from the Japanese clock. It holds no contact, so alert from belief does not set its flag at the start of the night pulse, and nothing sets it before the meeting. So at the meeting it is not alert and not ready, and under the live figures its guns fire at the unready share for the whole fight (decision 0068). Its mistaking the enemy for friends is not a separate rule; the model carries it only as not alert and not ready.
10. **Neither side starts with a contact.** The American air reports of 13:45 and 18:10 (engagement_events 565 to 569, 580 to 582) were of "two cruisers and six destroyers", which the sources read as the reinforcement group; the observers did not see Goto's three cruisers (558, 574). A contact of a force that is not in the scenario would be a belief of nothing; and Task Force 64 is alert by its readiness in any case. Nothing in the Japanese record says they had seen Task Force 64.
11. **Both forces engage. Their break-off levels are read from the rows, each apart, by the reading being written into decision 0024 for Savo Island:** a break-off level is the commander's order. A row stating an order to hold or fight on, or failing that the force's recorded conduct with no part of it leaving by choice at any loss, sets the level at 1. A row stating that the force withdrew by choice at a stated loss sets it at that share. A force with no such row keeps the default 0.5, marked invented. The bombardment group detaches a ship that cannot keep up, sent home to Shortland with an escort; Task Force 64 keeps its own. Every ship starts with full fuel and a full outfit (decision 0024, 10). The scenario was not run to see what either level does.

    - **Task Force 64: 1, by its conduct.** No row gives Scott an order to hold or fight on at any loss. His orders before the action are about the attack: the approach, illumination, torpedoes and gunfire (engagement_events 549, 550, 553), the attack that evening (555), and "ships out of formation drop out on the disengaged side" (552), a rule for a ship, not a withdrawal of the force. His conduct: the force fought from 23:46 until the enemy was gone. The cease-fire of 23:47 was ordered because he took his targets for his own destroyers (674, 675, 803), and firing resumed by 23:51 (681, 801, 1037); the one at about midnight was to rectify the formation (736, 806), and firing resumed at 00:04 (806). At 00:16 he turned to close the enemy (699). Only "after the last desultory firing" did he decide to retire, at 00:27 (699, 701), when San Francisco had ceased fire on the last target at 00:20 (700). Two ships left the line, and neither by choice: Boise fell out at 00:13 with her magazines on fire (696, 693), and Duncan left the column to attack, towards the enemy (532 in engagement_positions), and was abandoned on fire. No part of the force left by choice at any loss, so the level is 1. The orders and course changes of the night, read again by the research of Nov 13, 2026 (engagement_events 1222 to 1240; `docs/research/oob/orders-and-conduct.md`), agree: the cease-fire of 23:47 because he thought his cruisers were firing on his own destroyers (1227), recognition lights at 23:50 (1228), "commence firing" at 23:51 with no doubt left of the enemy (1229, 1237), cease fire and resume at 00:01 to 00:04 (1230), the turn of 00:05 (1231), and the turn away at 00:27 with an attempt to detail a destroyer to stand by Boise (1232, 1238). None is a withdrawal at a loss.
    - **The bombardment group: 0.5, invented. The architect's ruling of Nov 13, 2026, which replaces the reading of 0.2 below.** The reading's test is a row stating that the force withdrew by choice at a stated loss. It is not met. The orders-and-conduct research read the Japanese pages twice, blind, and both readings agree that no page names who gave the order to break off (outcomes 3834, 3835, 3837; the decision is written in the honorific passive); the older row that names Goto for the order of 23:43 (engagement_events 509) is now disputed; and the flash (534) gives as its reason that the bombardment had become impossible, not the loss, while the staff paper gives the commander's death, poor communication and a very unfavourable position (outcomes 3836). So no row gives a level, and the force keeps the default, 0.5, marked invented, with the note that no row gives a level. The reading of 0.2 is kept below as the record of how it was first read.
    - *Superseded: the bombardment group at 0.2, from its own flash signal.* The group's battle flash No. 6, sent by Aoba's captain (engagement_events 534, medium, preferred, the 4th Destroyer Squadron's copy), says it met the enemy at 21:45 (23:45 US), "fought 15 minutes and mostly beat them, but since shelling the shore became impossible all forces ordered to withdraw". That is a withdrawal of the force by choice. Fifteen minutes from 23:46 is about 00:01; by then Fubuki had sunk, at about 23:53 (the research report; her sinking rows), and she is one of the five. Furutaka had been hit and was burning from 23:50 (730) but was still under way and firing (731); the flash's own damage list ("Furutaka unable to proceed", "all Aoba's turrets unusable", Kinugasa sound) is as of its sending, and Furutaka's steering failed only at 00:40 (engagement_positions 504). So at the order one ship of five was sunk and none other was crippled by any row: the share is 1/5, and the level 0.2. The order was given after Goto was mortally wounded, by those who took command on Aoba (665, low; 528 says Aoba's captain took over formally at 06:00). Two things are written down so that the reading is not taken for more than it is. The flash gives the reason as the bombardment having become impossible, not the loss. And Aoba's first turn at 23:46 to 23:50, "for the moment to break away and put the force in order" (512), is not read as the withdrawal: the same row has her fighting on (gun and torpedo action to both sides), Kinugasa and Hatsuyuki turned and fought on for some 25 minutes (521, 525) and Furutaka fired from 23:52 (731). Were Furutaka counted from the flash's damage list, the share would be 2/5; she was not disabled when the order was given, so 0.2 stands.

    Both levels are the architect's reading of the rows, as decision 0024's are, and are to be reviewed with it. Neither is a source's figure.
12. **Ship order inside each force,** which decides who fires at whom (chapter 4, Who fires), is the order of decision 1: the American battle disposition of 22:35 and the Japanese column with its screen. The column turn that changed the American order is not modelled.

### The ships and their experience

13. **Experience is as derived (decision 0064),** `data/reference/ship_experience_cape_esperance.ships.csv`: San Francisco, Boise, Salt Lake City 3; Helena, Farenholt, Laffey, Buchanan 2; Duncan, McCalla 1; the five Japanese ships 3. Not to be changed.

## The ships' class figures

`data/derivation/cape_esperance.toml` already derives the ships' experience. With the facts as they stood, the derivation of their classes gave 48 problems. This stage settled the disputes that have a reason, and 14 problems remain, none of which can be closed without a change to `data/derivation/rules.toml`, decision 0070, or research. **No class file is derived yet,** and the class list stays a comment in the list file: `crates/oob-derive/tests/experience.rs` derives the whole file and fails on any problem, so enabling a list with problems breaks the build.

**The rates of fire are pending decision 0070,** which is changing the rate-of-fire rule in `rules.toml`. Every class file of this list is derived after it lands, in any case.

### Settled here

Each row's note says why, marked as settled for this decision by the architect's team lead and to be reviewed. Rows that agree with the chosen value became `preferred`, the others `alternative` (database design, "Conflicting sources"). `docs/research/oob/conflicts.md` lists them.

| Class or ship | Fact | Chosen | Rows preferred; alternative | Reason, in short |
| --- | --- | --- | --- | --- |
| Brooklyn (Boise) | Speed | 32.5 kn | 1028 to 1030; 1031 | Three sources over a lone DANFS header |
| St. Louis (Helena) | Speed | 32.5 kn | 1068 to 1070; 1071 | The same |
| Pensacola (Salt Lake City) | Speed | 32.5 kn | 992 to 994; 995 | The same |
| Brooklyn | Standard displacement | 9,700 t | 1017, 1018; 1019 | Two medium sources for Boise herself |
| St. Louis | Standard displacement | 10,000 t | 1055 to 1057; 1058 | Three sources; Navypedia's 10,560 may be the true figure (a known weakness) |
| Pensacola | Torpedo tubes | None | fits 676, 677; 675 | Removed in the mid-1930s |
| Gleaves, Benson | Rounds a gun | 360 | 1114, 1111; 1115, 1112 | Navweaps's own note: outfits raised to the design figure after 1940 |
| Benson (Laffey) | Speed | 35 kn | 1100; 1101 | The designed speed, as for every class, over a trial figure |
| Fubuki (Fubuki, Hatsuyuki) | Standard displacement | 2,090 t | 1123; 1124, 40 | After the rebuild of 1935 to 1937 |
| Fubuki | Speed | 34 kn | 1126; 44, 1127 | After the rebuild |
| Fubuki | Fuel | 475 t | 1128; 42, 1129 | After the rebuild |
| Fubuki | Endurance | 4,700 nm at 15 kn | 1130, 1132; 1131, 1133 | The medium source, read as a pair |
| Fubuki | Rounds a gun | 150 | 1136; 1137 | The wartime outfit |
| Buchanan | Torpedo tubes | Ten | fits 722; 723 | NavSource, as for her three sisters in the force; DANFS's five is undated (a known weakness) |

The Savo Island, Eastern Solomons and Rabaul lists derive exactly as before, and `oob-check` passes.

With these and the radar list below, **seven ships derive cleanly now:** Farenholt, Duncan, Buchanan, McCalla, Aoba, Furutaka, Kinugasa (tried on a copy of the list, not committed).

**Radar of unknown fit (decision 0019, 5).** No search-radar fit row holds on the date for Laffey, Buchanan, Duncan, McCalla and the five Japanese ships; they go on the list's `radar_unknown_means_none`. For the destroyers this changes nothing the rules read: a force sights with radar if any ship in it has a class with radar (`crates/sim-core/src/surface.rs`, `has_radar`), and Boise, Helena and Salt Lake City have a sourced surface-search set. Duncan's two rows that say she had one are low and not preferred (fits 662, 663). For the Japanese ships the rows for the date are Wikipedia's "none" alone (fits 664 to 667, low); the rows of Savo Island two months before, from Bates and the post-war interrogations, also say none (fits 281, 283, 284, 358, 360, 361), but are dated 8 August. Reading them as not fitted agrees with every row read.

### What is left, and what closes it

*As it stood after the first stage. Every item was closed in the second stage, by research, by the architect's rulings and by exceptions: see "Built", at the end.*

| Ship | Problem | What closes it | Recommended |
| --- | --- | --- | --- |
| San Francisco | Main battery: only the class row (fits 246: nine 8-inch/55, mark inferred, low) | An exception reading row 246, or a ship-level row | Exception, as decisions 0023 e4 and e5 did for her sisters. Research is cheap too (DANFS) |
| Boise, Helena | Fuel: the upper end of a range for both classes (rows 1035, 1075, inferred, low) | The Navy's fuel tables (FTP 218), as the Eastern Solomons classes were given | Research first; an exception if not found |
| Salt Lake City | Fuel: the upper end of a range (row 999) | The same | The same |
| Salt Lake City | Rounds a gun: 150 for the treaty cruisers' gun, not specific to her class (row 1016) | An exception reading row 1016 | Exception. She fired 296 rounds from ten guns, a fifth of 150 a gun, so the figure does not limit the night |
| Boise, Helena | Torpedo tubes: none, read from absence (fits 701, 712) | An exception for each, or a source that states none | Exception: no ship of either class had tubes |
| Fubuki, Hatsuyuki | Torpedo model in 1942: Type 90 (fits 726, low, "later replaced", undated) against the as-built Type 8 | A second source for the Type 90, or an exception | Research: Navweaps lists the Type 90 for the Fubuki class (the note on weapon_facts 410), which would make two agreeing sources. Exception if the page does not say so |
| Fubuki, Hatsuyuki | Practical rate of the 12.7 cm/50 (weapon_facts 480: 5.0, 481: 4.4) | Decision 0070 | Wait for 0070 |
| Laffey | No class fuel row; displacement on Wikipedia alone (1093); main battery four or five guns, disputed (fits 713, 724) | Research | Research: Friedman's US Destroyers, or the Navy's fuel tables and her own history. Not settled here: NavSource is the better graded, Wikipedia the more specific to her hull number, and neither reason is enough |

The new gaps are in `data/oob/gaps.csv`.

## What the rules have, and do not have, here

Read from `docs/rules/04-surface-combat-and-damage.md` and chapter 3, and checked in the code.

- **Radar.** One rule reads it: sighting. A force's chance to sight in a round is `min(1, (base + radar_bonus) x alert_factor x experience_factor) x fatigue_factor`, and `radar_bonus` is `surface_combat.toml: radar_sighting_bonus` if any ship afloat in the force has a class with `radar` true (chapter 4, Sighting; `surface.rs`, `sight` and `has_radar`). Radar is a yes or no for a class, set by the derivation from a search-radar fit, air or surface (`rules.toml`, the radar facts read). It has no range, does not enter the hit chance (chapter 4, The hit chance: "Radar does not enter the hit chance"), is not damaged, and a set switched off still counts: San Francisco's air-search set, ordered not to be used (engagement_events 586), makes her class row read radar. Nothing reads which ship had the set or whether its report reached the admiral.
- **Surprise.** A force is surprised in a round if it has not sighted, the enemy has, and the enemy has shots to fire (chapter 4, Surprise). A surprised force does not fire, and fire at it is more accurate (`surprised_target_factor`). A force fired on sights the next round with no roll.
- **Alertness and readiness.** A force not alert sights at `not_alert_sighting_factor` of the chance; a force not ready fires its guns at `unready_gun_share` for the whole fight (decisions 0021, 0068). Here the bombardment group is both.
- **What a test may assert, on the frozen figures** (`data/reference/step1/surface_combat.toml`): in round 1, Task Force 64's chance to sight is (0.5 + 0.2) x 1 = 0.7, and the bombardment group's 0.5 x 0.5 = 0.25 (experience reads nothing there, the strength being 0). On the live figures the experience factors are 1 + 0.25 x (19/9 - 2) = 1.027777 for Task Force 64 (the mean of 3, 3, 3, 2, 2, 1, 2, 2, 1 is 2.111111) and 1 + 0.25 x (3 - 2) = 1.25 for the group. These are chances, not outcomes.

**The events a measure of who sighted whom first can read.** A force that sights by its own roll records `SightingRoll` with `sighted` true and the round (`crates/sim-core/src/events.rs`). A force that sights because it was fired on records no roll: it is the force `ForceSurprised` named in the round before, and a `Salvo` at one of its ships stands in that round. `EngagementBegun` gives the meeting. A force at normal readiness that sights or is fired on also records `ForceBecameAlert`. So "who sighted first" is the force with the earlier round of a successful `SightingRoll`, or the one that surprised the other; both in one round is a tie.

## What the rules cannot show here

The night turned on several things no rule covers. None is designed here.

| What happened | What the rules do | Recommendation |
| --- | --- | --- |
| **Radar fire control.** Helena and Boise laid their guns by radar at about 4,000 yards; the American cruisers hit the Japanese cruisers some 130 times in 34 minutes | Radar enters sighting only; a hit is as likely with or without it | A rule may be needed: the spec's argument for experience holds here too, since tuning on Savo Island (where the side with radar fired almost nothing) cannot set it. **The architect ruled on Nov 13, 2026 not to build it before the check is run.** Accepted and noted with the others; if Cape Esperance misses for it when it is run, that is written as the cause and a plan follows then |
| **Fire on a friend.** Farenholt and Duncan were hit by their own cruisers; Scott ceased fire believing he was hitting his own destroyers | No fire on a friend; no cease-fire | Accept and note. The measure of hits on American ships is already unjudged for this reason (`data/validation/cape_esperance.toml`) |
| **The column turn.** The countermarch at 23:32 to 23:36 left three destroyers between the lines | No formation or positions inside a hex | Accept and note |
| **Seeing and recognising.** Each side saw the other before fire opened; the Japanese took the Americans for their own landing group | Sighting is recognition. The Japanese mistake is carried only as not alert and not ready | Accept and note. If later battles turn on it too, an identification rule is a candidate |
| **Radar by ship and the report's delay.** The flagship had no surface set; Helena's contact took minutes to reach Scott | Radar is a yes or no for the force | Accept and note |
| **Range.** Fire opened at 4,000 yards | No range; one night hit chance | Accept and note (a known limit of chapter 4) |
| **Illumination.** Boise's searchlight at 23:53 drew fire on her | Only fire damage lights a ship | Accept and note, as at Savo Island (decision 0024, deferred) |
| **Command.** Goto was mortally wounded by the first salvo and the group turned away | A force breaks off by a share of ships lost | Accept and note |
| **Ships acting apart.** Kinugasa and Hatsuyuki turned away separately and fought on; Duncan attacked alone | One force, ships paired by order in the list | Accept and note |
| **Magazine fire and loss after the fight.** Boise's magazines burned; Duncan sank next day after a failed tow | Flotation of 100 sinks a ship; nothing else; no magazine explosion | Accept and note |
| **The aircraft.** Floatplanes reported the reinforcement group and one burned on the water | Not in the scenario | Accept and note |
| **The day after.** Murakumo and Natsugumo were sunk by aircraft | Out of scope (decision 2) | Accept and note |

## What is a placeholder

- Every combat figure in `surface_combat.toml` and the objectives scale, as in every scenario until step 2 sets them.
- The bombardment group's break-off level, 0.5: invented, since no row gives one (decision 11). Task Force 64's level of 1 is read from its conduct.
- Full fuel and full outfits at the start (decision 0024, 10).
- The 18:00 positions of both forces and the Japanese speed of 27 knots, worked back as decisions 5 and 6 say. They are the architect's settlement, not a source's statement.
- Laffey's guns, displacement and fuel, read by exception and by inference ("Built", below), until a better source is found.

## Gaps

- The class figures still read by exception or inference ("Built", below), each open in `gaps.csv`.
- Positions of either force in latitude and longitude near the contact, and column intervals (the research report).
- Rounds fired and hits by ship; the American ships' action reports are not online (the research report). The measures already allow for it.
- Torpedoes fired by the American ships: no count for any (Buchanan and Duncan attacked; Farenholt's mount jammed at 23:50). The Japanese counts are Aoba's 2 and Kinugasa's 4 (engagement_outcomes 2277, 2282). Research pull request 174 added the tubes that bear on one side for each class (ship_class_facts 1176 to 1353, `docs/research/oob/torpedoes-fired.md`); no rule reads them yet (decision 0074). *(Nov 14, 2026: decision 0083, built with decision 0074, makes a ship fire the tubes that bear on one side first and the other side in her next torpedo round)*
- The Japanese group's state at contact beyond what decision 9 cites: what its guns were loaded with, where they were trained, and whether a recognition signal was made (the flagship's signal log was lost). These stay gaps; decision 9 does not rest on them.

## Built, Nov 13, 2026 (the second stage)

The ships are derived and the scenario is written. **Nothing ran the battle to an outcome that anyone reads:** no batch, no `validate`, no events printed past the meeting, and no run baseline.

### The architect's rulings of Nov 13, 2026

1. **No rule of radar fire control before the check is run** (see "What the rules cannot show here").
2. **The class file is copied into `data/reference/step1`** when it is first derived, by "Tests while figures move", 6: what step 2 adds goes into the frozen copy too. Scenario 17's test runs on the frozen figures like the others. The step1 README says so.
3. **The bombardment group's break-off level is 0.5, invented** (decision 11 above).
4. **Laffey is in, and the scenario does not go to 13 ships.** Leaving out a ship that fought changes the battle more than a marked settlement does. Her three figures are read by exception (below) and stay open in `gaps.csv`. The architect has asked research for Friedman on all three; if it contradicts, she is re-derived before the scenario is ever run.
5. **The Fubuki class's torpedo** comes from the fit row that has a count (726), by exception. The type-only statements are now `torpedo_model` rows (ship_class_facts 1359 to 1362, research pull request 198).

### The data settled in this stage

Each settlement has its reason on the row or in `rules.toml`. Rows changed: standing and note only.

- **Fuel, read alike for every ship.** Savo Island and the Eastern Solomons read FTP 218 Table V's "Full load" (for example rows 712, 864, 877), with endurance from the 15-knot row (834, 862, 875). The same was read here, from the HyperWar transcription on Nov 13, 2026:

  | Class | Full load | Endurance at 15 kn | Rows |
  | --- | --- | --- | --- |
  | Brooklyn (CL 47, Boise's own table) | 1,981 t | 7,990 miles (Table I, radius-oil basis) | 1363 to 1365 |
  | St. Louis (CL 49 alone; read for Helena by inference) | 1,892 t | 7,690 miles (Table I) | 1366 to 1368 |
  | Pensacola (CA 24 and 25) | 2,116 t | 7,020 miles (Table I) | 1369 to 1371 |
  | Benson (DD 421/459, which does not list Laffey; read for her as a ship of the class, by inference) | 466 t | 5,100 miles (Table IAA; radius oil and maximum capacity are one figure for this class) | 1372 to 1374 |

  The maximum-capacity rows (1354 to 1357) and the earlier range-ends and design endurances became `alternative`. Row 999 (Navypedia's 2,116 for the Pensacola class) agrees with the full load and became `preferred`.
- **The 12.7 cm/50's rate (Fubuki, Hatsuyuki):** row 480 (5.0 a minute, from the US Naval Technical Mission's figure for the Type A twin mount) is preferred over 481 (Lengerer's 4.4 to 5.0, lower end). The technical examination outranks the compilation. Under decision 0070 the class fires at the lesser of this and its kind's achieved rate.
- **San Francisco's main battery** is now read from her own row (fits 756, DANFS, medium, preferred), with no exception.

The exceptions, appended at the end of `rules.toml`:

| Ship or class | Fact | Row | In short |
| --- | --- | --- | --- |
| Laffey | Main battery, four 5-inch | fits 749 | The two rows dated at or near the action say four: 748, the Cominch bulletin's table of 13 November, a primary source not itself read as it is dated after the list's day; and 749, DANFS. The two that say five are a launch-time caption (750) and an undated compilation (713). Low confidence. The architect's ruling |
| Benson class (Laffey) | Standard displacement, 1,620 t | 1093 | A lone row; every source calls the class 1,620-ton. The architect's ruling |
| Brooklyn class (Boise) | No torpedo tubes | fits 701 | No armament line lists any |
| St. Louis class (Helena) | No torpedo tubes | fits 712 | As above |
| Pensacola class (Salt Lake City) | 150 rounds a gun | 1016 | Not specific to the class. The list's `main_gun_ammunition_not_limiting` is not applied over a row, so the row is read. She fired 296 of 1,500, so it does not limit her |
| Fubuki class | Nine tubes, Type 90 | fits 726 | Every source agrees on the type (1359 to 1362), and none disputes the count (727 to 729). The architect's ruling |

**Radar under decision 0077:**
- Boise and Helena read 1 (SG, a plan position indicator).
- San Francisco reads 0 (no SG; her SC is an air-search set).
- Farenholt's set of type not stated is named as not counted.
- **Duncan.** Her two disputed rows do not stop the derivation. The tool finds no surface-search row that holds and reports her radar as unknown. She is on the list's `radar_unknown_means_none` and is named there (0077, rule 5).
- **Salt Lake City reads 0 and is named nowhere.** Her one surface-search row for the date (fits 649, the SG, "so, presumably") is disputed, so it is not read. Her air-search fit is read, so a radar fit is on record, and the tool gives 0 without a note. Rule 3's naming of a set it does not count does not reach a set whose row is disputed. That is how the tool behaves, reported here and not changed. For the force it changes nothing, since Boise and Helena have the figure.

**The class ids are pinned** (decision 0077) in `data/derivation/cape_esperance.toml`. They are 3000 to 3008, which no other list uses.

| Id | Class | Ships | Top speed | Radar |
| --- | --- | --- | --- | --- |
| 3000 | Aoba | Aoba, Kinugasa | 33 | 0 |
| 3001 | Furutaka | Furutaka | 33 | 0 |
| 3002 | Fubuki | Fubuki, Hatsuyuki | 34 | 0 |
| 3003 | Gleaves | Farenholt, Duncan, Buchanan, McCalla | 35 | 0 |
| 3004 | Benson | Laffey | 35 | 0 |
| 3005 | New Orleans | San Francisco | 32.7 | 0 |
| 3006 | Brooklyn | Boise | 32.5 | 1 |
| 3007 | Pensacola | Salt Lake City | 32.5 | 0 |
| 3008 | St. Louis | Helena | 32.5 | 1 |

Every top speed is above both ordered speeds, 27 and 29 knots, so the hand calculation of the meeting holds on the derived classes. The experience file did not change.

FTP 218 lists DD 491, Farenholt, in its DD 421/459 (Benson) table. That bears on the open Benson-or-Gleaves question for her (`gaps.csv`). She is left in the Gleaves class, as the rows have her.

### The scenario and its test

- **`data/scenarios/not-run/17_cape_esperance.toml`, not `data/scenarios/`.** Every scenario in `data/scenarios/*.toml` is walked by three loops:
  - CI's export of every scenario;
  - `crates/sim-cli/tests/export.rs`, which requires a golden of every scenario's full run in `crates/sim-cli/tests/run_baseline`;
  - `crates/sim-cli/tests/saved_games.rs`.

  A golden would record the outcome. None of the three looks in a sub-folder, as with `step2-frozen/`. The file moves to `data/scenarios/` when the scenario may be run, after Savo Island is tuned.
- **Scenario 17 is free;** the test file `tests/scenario_17a_target_choice.rs` is decision 0078's and has no scenario file of its own.
- **Shortland's hex (-2,-8)** takes the base: the scenario loads with it.
- **Ids,** for the measures' `take` keys:
  - task force 1, Japanese, at normal readiness: ships 1 Aoba, 2 Furutaka, 3 Kinugasa, 4 Fubuki, 5 Hatsuyuki.
  - task force 2, Allied, at battle readiness: ships 6 Farenholt, 7 Duncan, 8 Laffey, 9 San Francisco, 10 Boise, 11 Salt Lake City, 12 Helena, 13 Buchanan, 14 McCalla.
  - base 1, Shortland.
- **Its test,** `tests/scenario_17_cape_esperance.rs`, runs the one day on the frozen figures and reads only the state as loaded and the events up to the first `EngagementBegun`. Its hand calculation is in its doc comment. It proves:
  - the scenario loads;
  - the ship order and classes;
  - the readiness as loaded;
  - the two approaches;
  - the one meeting of the night: forces 1 and 2, hex (2,-5), 5.703703 hours (23:42), dark. Force 2 has stood in (2,-5) since it entered at 22:20, after 126 miles. Its last 20 miles to the centre and its arrival (by the clock 23:02) are recorded after the fight, by decision 0022, 6, so the test does not read them; the first stage's "arrives at 23:02 before the meeting" was wrong as to the order of the events;
  - no alert of force 1 before it.

**Who fires at whom, by decision 0078.** This replaces the pairing by place of the first stage. The test does not assert it: who fires in a round depends on who has sighted, which is drawn, and reading salvos would read the fight. It is worked here for the record, for round 1 with every ship afloat.

- **Guns, like at like.** A ship takes the ship in place (p mod n) among the enemy ships of its own kind, where p is its place among its own kind in its force.
  - American cruisers at the Japanese cruisers: San Francisco at Aoba, Boise at Furutaka, Salt Lake City at Kinugasa, Helena at Aoba.
  - American destroyers at the Japanese destroyers: Farenholt at Fubuki, Duncan at Hatsuyuki, Laffey at Fubuki, Buchanan at Hatsuyuki, McCalla at Fubuki.
  - Japanese cruisers at the American cruisers: Aoba at San Francisco, Furutaka at Boise, Kinugasa at Salt Lake City.
  - Japanese destroyers at the American destroyers: Fubuki at Farenholt, Hatsuyuki at Duncan.
- **Torpedoes, at the most important kind afloat, the cruisers.** A ship takes the ship in place (i mod n), where i is its place in its force.
  - American ships with tubes: Farenholt (i = 0) at Aoba, Duncan (1) at Furutaka, Laffey (2) at Kinugasa, Buchanan (7) at Furutaka, McCalla (8) at Kinugasa. The four American cruisers have no tubes.
  - Japanese ships: Aoba (0) at San Francisco, Furutaka (1) at Boise, Kinugasa (2) at Salt Lake City, Fubuki (3) at Helena, Hatsuyuki (4) at San Francisco.

## Why

- **The reinforcement group out** keeps the battle that was fought. Left in, the rules would make a fight that did not happen, before the one that did. It is also the scope the measures assume.
- **Starting positions between the sources, and a speed worked back to the hour,** is how Savo Island was built, and the two scenarios are to be read against each other.
- **Settling the class disputes now** leaves the next stage with exceptions and research only, each listed with its rows.

## Rule card

No rule. These are scenario inputs and data settlements.

## As built (the check), Nov 13, 2026

The third stage: the one run of the check, committed in advance by decision 0084 (`docs/step2-spec.md`, "Tuning"). Nothing was tuned on it and nothing is changed by it; a rule or figure changed after it is not checked by it.

**Before the batch, in one commit (897ba4f, on a branch cut from `main` at 3e90304):**
- The scenario moved from `data/scenarios/not-run/` to `data/scenarios/17_cape_esperance.toml`. Its six data paths lost one `../`, since they are read from the scenario's own folder, and its header paragraph was rewritten. No figure, order, ship or other key changed.
- `data/validation/cape_esperance.toml` got its `scenario` key and a `take` for each of its eleven measures (ships 1 to 5 the Japanese, 9 to 12 the US cruisers, 6, 7, 8, 13 and 14 the US destroyers, as "Ids" above), with a dated line at its head. No record, row, range, reason or judged flag changed. No measure was added: who sighted whom first stays unwritten.
- Scenario 17's test reads the new path; the test of `validate` on a measures file with no scenario uses a hand-made file in place of this one.

Nothing of the scenario was run before the batch but its hand-calculation test.

**The commands, once each:**

```
cargo run --release -p sim-cli -- batch data/scenarios/17_cape_esperance.toml --runs 1000 --first-seed 1 --label check --out cape_esperance_check.json
cargo run --release -p sim-cli -- validate cape_esperance_check.json data/validation/cape_esperance.toml
```

**Hashes** (as the report gives them; the batch file is not kept in the repository, as for the other battles):
- The batch file: 100525c6c713b215 (SHA-256 c0c7ebd37390d4c4d6ef076654664aef75bb71c365fc703bd3c0684a77a42728).
- State hash layout flagplot-state-v8.
- The scenario file: 6984fa7a6979aa28.
- `south_pacific.csv` ba6cb3aea5cc45f5; `ship_classes_cape_esperance.csv` c79b1bc1e17fcc16; `port_sizes.csv` da12e2b751976274; `aircraft_types.csv` 92830cbacc0a4328; `search.toml` 9d9cd2961a30d99a; `surface_combat.toml` 1a9a0ae06d7282b1.
- The measures file: dcda49f98a4431be.

**The result** (`docs/validation/cape-esperance-check.md`): judged 8, agree 4, at the edge 1, miss 3. The battle does not pass.

| Measure | Record | 5th to 95th | Median | Runs holding the record | Left out | Verdict |
| --- | --- | --- | --- | --- | --- | --- |
| japanese_ships_sunk | 2 | 0 to 3 | 2 | 407 of 1000 | 0 | agrees |
| japanese_ships_with_propulsion_out | 0 | 0 to 1 | 0 | 908 of 1000 | 0 | agrees |
| us_cruisers_sunk | 0 | 0 to 0 | 0 | 960 of 1000 | 0 | agrees |
| us_destroyers_sunk | 1 | 0 to 0 | 0 | 4 of 1000 | 0 | **MISS**: the record is outside the middle 90 percent; fewer than 1 run in 20 holds the record |
| gun_hits_on_aoba | 40 | 13 to 55 | 29 | 18 of 1000 | 0 | **MISS**: fewer than 1 run in 20 holds the record |
| gun_hits_on_furutaka | 90 | 10 to 57 | 27 | 0 of 1000 | 0 | reported |
| gun_hits_on_kinugasa | 0 to 4 | 4 to 49 | 15 | 60 of 1000 | 0 | agrees, at the edge |
| gun_hits_on_us_ships | 15 to 29 | 1 to 35 | 6 | 179 of 1000 | 0 | reported |
| torpedo_hits_on_japanese_ships | 0 to 1 | 0 to 6 | 3 | 269 of 1000 | 0 | agrees |
| japanese_ships_claimed_sunk_per_hundred_sunk | 200 to 300 | 100 to 300 | 100 | 2 of 44 | 956 | **MISS**: too few runs kept |
| us_ships_claimed_sunk_per_hundred_sunk | 400 to 750 | 100 to 250 | 100 | 8 of 923 | 77 | reported |

`docs/validation/baseline-reading.md` ("Cape Esperance: the check") reads it: what it shows, what it says of the three figures fitted at Savo Island within decision 0084's limits, and, marked as conjecture for step 3, what each miss most likely rests on. Two things the scenario does that the record does not, found in the batch and named there: Task Force 64 fights Japanese stragglers by day in 771 runs, since it is ordered to stay in (2,-5); and its five destroyers fire every tube. Neither is measured.

The scenario now sits in `data/scenarios/`, so every loop over that folder runs it: CI's export, the run baseline of `crates/sim-cli/tests/export.rs` and the saved-games test. Those files were made after the check, from the same scenario; they record its outcome and were not read for the check.
