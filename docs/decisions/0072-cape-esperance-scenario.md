# 0072: The Cape Esperance scenario: what is in it and what is left out

Status: plan; settled by the architect, to be reviewed. Drafted by the architect's team lead on Nov 13, 2026. The data choices below are settled as the architect's (decision 0015 and `docs/engineering/data-rules.md`, "Who settles data") and wait for the architect's review. No rule is added or changed.

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

    - **Task Force 64: 1, by its conduct.** No row gives Scott an order to hold or fight on at any loss. His orders before the action are about the attack: the approach, illumination, torpedoes and gunfire (engagement_events 549, 550, 553), the attack that evening (555), and "ships out of formation drop out on the disengaged side" (552), a rule for a ship, not a withdrawal of the force. His conduct: the force fought from 23:46 until the enemy was gone. The cease-fire of 23:47 was ordered because he took his targets for his own destroyers (674, 675, 803), and firing resumed by 23:51 (681, 801, 1037); the one at about midnight was to rectify the formation (736, 806), and firing resumed at 00:04 (806). At 00:16 he turned to close the enemy (699). Only "after the last desultory firing" did he decide to retire, at 00:27 (699, 701), when San Francisco had ceased fire on the last target at 00:20 (700). Two ships left the line, and neither by choice: Boise fell out at 00:13 with her magazines on fire (696, 693), and Duncan left the column to attack, towards the enemy (532 in engagement_positions), and was abandoned on fire. No part of the force left by choice at any loss, so the level is 1.
    - **The bombardment group: 0.2, from its own flash signal.** The group's battle flash No. 6, sent by Aoba's captain (engagement_events 534, medium, preferred, the 4th Destroyer Squadron's copy), says it met the enemy at 21:45 (23:45 US), "fought 15 minutes and mostly beat them, but since shelling the shore became impossible all forces ordered to withdraw". That is a withdrawal of the force by choice. Fifteen minutes from 23:46 is about 00:01; by then Fubuki had sunk, at about 23:53 (the research report; her sinking rows), and she is one of the five. Furutaka had been hit and was burning from 23:50 (730) but was still under way and firing (731); the flash's own damage list ("Furutaka unable to proceed", "all Aoba's turrets unusable", Kinugasa sound) is as of its sending, and Furutaka's steering failed only at 00:40 (engagement_positions 504). So at the order one ship of five was sunk and none other was crippled by any row: the share is 1/5, and the level 0.2. The order was given after Goto was mortally wounded, by those who took command on Aoba (665, low; 528 says Aoba's captain took over formally at 06:00). Two things are written down so that the reading is not taken for more than it is. The flash gives the reason as the bombardment having become impossible, not the loss. And Aoba's first turn at 23:46 to 23:50, "for the moment to break away and put the force in order" (512), is not read as the withdrawal: the same row has her fighting on (gun and torpedo action to both sides), Kinugasa and Hatsuyuki turned and fought on for some 25 minutes (521, 525) and Furutaka fired from 23:52 (731). Were Furutaka counted from the flash's damage list, the share would be 2/5; she was not disabled when the order was given, so 0.2 stands.

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
| **Radar fire control.** Helena and Boise laid their guns by radar at about 4,000 yards; the American cruisers hit the Japanese cruisers some 130 times in 34 minutes | Radar enters sighting only; a hit is as likely with or without it | **A rule may be needed, and the architect should decide before Savo Island is tuned.** The spec's argument for experience holds here too: if radar fire control is not a rule, tuning on Savo Island (where the side with radar fired almost nothing) cannot set it, and Cape Esperance then misses for a reason no figure can mend. Flag for the architect |
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

- Every combat figure in `surface_combat.toml` and the objectives scale, as in every scenario until step 2 sets them. The break-off levels are not placeholders: they are read from rows (decision 11), as the architect's reading.
- Full fuel and full outfits at the start (decision 0024, 10).
- The 18:00 positions of both forces and the Japanese speed of 27 knots, worked back as decisions 5 and 6 say. They are the architect's settlement, not a source's statement.
- The rates of fire, until decision 0070 lands and the classes are derived.

## Gaps

- The class figures of decision "What is left", above; the new rows in `gaps.csv`.
- Positions of either force in latitude and longitude near the contact, and column intervals (the research report).
- Rounds fired and hits by ship; the American ships' action reports are not online (the research report). The measures already allow for it.
- Torpedoes fired by the American ships: no count for any (Buchanan and Duncan attacked; Farenholt's mount jammed at 23:50). The Japanese counts are Aoba's 2 and Kinugasa's 4 (engagement_outcomes 2277, 2282). Research pull request 174, not merged when this was written, adds the tubes that bear on one side for each class (ship_class_facts 1176 to 1353 there); no rule reads them yet.
- The Japanese group's state at contact beyond what decision 9 cites: what its guns were loaded with, where they were trained, and whether a recognition signal was made (the flagship's signal log was lost). These stay gaps; decision 9 does not rest on them.

## For the next stage

The scenario file is left for the next stage, for two reasons:

1. **It cannot load without a class file,** which waits on the table above.
2. **Its test needs a decision.** A scenario test loads through `tests/common` on the frozen figures of step 1 (`docs/engineering/roles-and-tests.md`), and `--reference data/reference/step1` reads every reference file the scenario names from that directory, so `ship_classes_cape_esperance.csv` would have to be there. It is not a figure of step 1. The architect decides whether the class file is copied into the frozen directory when it is derived, or the test loads on the live figures (`common::load_live`) and asserts only what no figure decides: the meeting, the readiness and the order of the ships, not the sighting chances.

When it is built, it is `data/scenarios/17_cape_esperance.toml` (16 is being built by another session; check `origin/main` again first), with these task forces and ships, the ids being what the measures' `take` keys are written against:

| Id | Task force | Ship | Class (to be derived) | Experience |
| --- | --- | --- | --- | --- |
| TF 1 | Bombardment group, Japanese, normal readiness | | | |
| 1 | 1 | Aoba | Aoba class | 3 |
| 2 | 1 | Furutaka | Furutaka class | 3 |
| 3 | 1 | Kinugasa | Aoba class | 3 |
| 4 | 1 | Fubuki | Fubuki class | 3 |
| 5 | 1 | Hatsuyuki | Fubuki class | 3 |
| TF 2 | Task Force 64, Allied, battle readiness | | | |
| 6 | 2 | USS Farenholt | Gleaves class | 2 |
| 7 | 2 | USS Duncan | Gleaves class | 1 |
| 8 | 2 | USS Laffey | Benson class | 2 |
| 9 | 2 | USS San Francisco | New Orleans class | 3 |
| 10 | 2 | USS Boise | Brooklyn class | 3 |
| 11 | 2 | USS Salt Lake City | Pensacola class | 3 |
| 12 | 2 | USS Helena | St. Louis class | 2 |
| 13 | 2 | USS Buchanan | Gleaves class | 2 |
| 14 | 2 | USS McCalla | Gleaves class | 1 |

Base 1 is Shortland, Japanese, hex (-2,-8), holding nothing, as Rabaul is in scenario 4; that the hex takes a base is to be checked when the file is written.

The hand calculation its test asserts, all of it fixed by orders, the map and the readiness, none of it by a draw:

- Task Force 2 from (2,-2) at 29 knots: steps (2,-3), (1,-3), (1,-4) charged 37, 29 and 40 (106, the air distance), then (2,-5) charged 40. It enters (2,-5) after 126 miles, 4.344827 hours (22:20), and arrives at its centre after 146, 5.034482 hours (23:02).
- Task Force 1 from (0,-8) at 27 knots: steps charged 30, 38, 26, 40, 40. It enters (2,-6) after 114 miles (4.222222 hours, 22:13) and (2,-5) after 154 (5.703703 hours, 23:42).
- One meeting, Task Force 1 and Task Force 2, night pulse of day 1, hex (2,-5), elapsed 5.703703 hours, dark. No other `EngagementBegun` in the game's night pulse.
- At the meeting Task Force 2 is at battle readiness, alert and ready; Task Force 1 is at normal readiness, not alerted, not ready.
- In round 1, ship i of one force fires at ship (i mod n) of the other: Farenholt at Aoba, Duncan at Furutaka, Laffey at Kinugasa, San Francisco at Fubuki, Boise at Hatsuyuki, Salt Lake City at Aoba, Helena at Furutaka, Buchanan at Kinugasa, McCalla at Fubuki; Aoba at Farenholt, Furutaka at Duncan, Kinugasa at Laffey, Fubuki at San Francisco, Hatsuyuki at Boise. (Who fires in round 1 depends on who sighted, which is drawn; the pairing is fixed.)

The test asserts or prints no outcome: no hit, no loss, no claim.

## Why

- **The reinforcement group out** keeps the battle that was fought. Left in, the rules would make a fight that did not happen, before the one that did. It is also the scope the measures assume.
- **Starting positions between the sources, and a speed worked back to the hour,** is how Savo Island was built, and the two scenarios are to be read against each other.
- **Settling the class disputes now** leaves the next stage with exceptions and research only, each listed with its rows.

## Rule card

No rule. These are scenario inputs and data settlements.
