# Legibility test: the prompts and the model's answers

Both sessions were fresh subagents on the model `haiku` (Claude Haiku), run on Nov 13, 2026. The prompts and the answers below are word for word. The record is `../closing-tests.md`; the game the model was shown is `legibility-game.md`.

## Session 1, prediction

### Prompt P1

> You are the Allied commander's chief of staff in a wargame of the Eastern Solomons, 24 August 1942. In the folder [scratch]/ct_leg/s1 are three files: brief_open.txt, the briefing your side received at the start of the day; orders_answers.txt, the orders given (each issue_orders call) and the game's answer to each; and glossary.md, the game's glossary. Read those three files with the Read tool and nothing else: open no other file, run no program, use no web.
>
> The day has three pulses: night, morning, afternoon. Using only this text and the glossary, predict what the day will bring. Answer each, with the reason from the text:
> 1. Will your side's searches find the Japanese carriers? In which pulse? Which group, and what makes it able to?
> 2. Will any of your strike groups launch? At what, in which pulse, and why that target and not another?
> 3. Will the Japanese strike your carriers? If so, what decides how much harm it does?
> 4. Roughly how many of your aircraft do you expect to lose, and to what causes?
> 5. Which ship of yours is most at risk, and of what (sunk, deck out, propulsion out)?
> 6. What result on the objectives scale do you expect, and why could it be different?
> 7. Name anything in the briefing you could not understand or had to guess.
>
> Your final report must be your whole answer, exactly as you would hand it in.

([scratch] stands for the session's scratch folder; the folder held exactly the three files named, the glossary being `docs/glossary.md` at the tested commit.)

### Answer

Prediction for the Eastern Solomons day (one day, the last, resolved by end_day). Based only on brief_open.txt, orders_answers.txt and glossary.md. Confidence is low where I say so: the briefing gives no Japanese positions, forces or sighting odds.

1. Searches finding the carriers: possible, probably in the morning pulse, not certain. Three groups search, all at a 200 nm arc: air_group:3 VS-5 (Enterprise, 270 to 0 degrees, 16 ready), air_group:7 VS-3 (Saratoga, 0 to 90 degrees, 15 ready), air_group:10 VMSB-232 (Henderson, 300 to 60 degrees, 11 ready). Together they cover every bearing north of the carriers out to 200 nm. Whether a carrier sits inside that is not in the text; the briefing says only that it comes "down from the north". Sighting odds are not given (sweep width is in the glossary but no figure is in the files). I expect no sighting in the night pulse: the glossary says the night pulse is dark and light is asked of one function, though the files do not say searches stop at night. VS-3 is the most likely finder if the force is north-north-east, VS-5 if north-north-west.

2. Strike launches: yes if a contact is found in morning, from the carriers. Four naval_strike groups: VB-6 (air_group:2, Enterprise, radius 200), VT-3 (air_group:4, Enterprise, radius 190), VB-3 (air_group:6, Saratoga, radius 200), VT-8 (air_group:8, Saratoga, radius 190). The staff launches them in a daylight pulse at a held contact reported in that pulse or the one before, within the radius. Because the target lists are the default (all six kinds, carriers first), a carrier contact is struck first. Any other kind in reach is struck only if no carrier is held, in the order battleship, cruiser, destroyer, transport, auxiliary, then nearest. A second wave in the afternoon is possible if the contact is still held and aircraft are ready. The VT-3 and VT-8 "second" groups (air_group:11 and air_group:12) were left on rest, so they will not fly.

3. Japanese strike on the carriers: the files do not show any Japanese air group, so I cannot say whether one comes. It depends on whether their carriers find ours. If one comes, harm depends on: the fighters over the hex (VF-6, 27 ready, all on combat air patrol; VF-5, half of 27 on patrol; combined about 40); the anti-aircraft fire of both task forces, which is at full effect because both are at battle readiness and so ready and alert, not surprised; and the damage each hit does (100 times weapon damage over the target's toughness, per the glossary). Both carriers sit in the same hex (7,-5) and neither task force has a move order, so one attack can reach both.

4. Our losses: none yesterday. Today I expect few: zero to about three if nothing is found, and perhaps five to fifteen if a strike goes in against a carrier. The causes would be enemy fighters and anti-aircraft fire, and wear: a worn aircraft after each sortie can be written off, and the rate is not given. Ditching is unlikely, because no extended reach was ordered. Low confidence on the numbers.

5. Most at risk: Saratoga (ship:11), marginally. The two carriers have near-equal exposure and aircraft (Enterprise 74, Saratoga 70). Saratoga's task force is smaller (8 ships against 10), and only half of its fighters are on patrol. The likeliest serious outcome is flight deck out, not a sinking. Confidence is low.

6. Objectives result: most likely a stalemate (margin under 3), because no ship is likely sunk on either side. A sunk carrier would give 10 points and a major victory (margin 8 or more); a crippled carrier, 5 points and a minor victory. The margin could differ in four ways: a Japanese hit on one of our carriers counts for them (10 for a sinking, 5 for a crippled carrier); aircraft destroyed count 0.05 each, so a large air fight can move the margin; transports (2 each) and auxiliaries (1 each) in the reinforcement group count; and the result is worked from the truth, not from claims. Our claims may be well above what happened, and they do not change the result.

7. Things I could not understand or had to guess:
- No Japanese positions, forces, air strength or intentions beyond "from the north" and "supporting the reinforcement of Guadalcanal".
- Sighting odds and sweep width: the glossary defines sweep width but no value is given.
- Whether searches run in the night pulse at all.
- The search range: the arc was ordered at 200 nm, believing that to be the maximum. The probe (call 1) was refused with "at most the aircraft's 397 nm", so 397 is the cap and 200 was my own choice, not a limit. The radius (190 to 200) is what limits the strike, so the arcs may stop short of contacts the strike could not reach anyway.
- Whether air_group:11 and air_group:4 (both "VT-3", 7 ready) are the same aircraft counted twice, and likewise air_group:12 and air_group:8 (VT-8). If they are separate, the torpedo strike is larger than I have assumed.
- Whether Henderson's "fuel 0 t" grounds VMSB-232 and VMF-223. The briefing says base supply is not in play, which suggests not, but the glossary's out-of-supply test is for supply stock, not fuel.
- How a contact's kind is shown when a search reports it, and whether a carrier can be told apart from another kind on a single report.
- What a task force mission does: the glossary says none is read by any rule yet.
- The briefing's "history note" is not in the files, so I did not use history.
- The one-day game: all orders must be in before end_day, and end_day ends the game.

## Session 2, explanation

### Prompt P2

> You are the Allied commander's chief of staff in a wargame of the Eastern Solomons, 24 August 1942. The day has been played and the game is over. In the folder [scratch]/ct_leg/s2 are: brief_open.txt (the briefing your side had at the start of the day), orders_answers.txt (the orders given and the game's answer to each), prediction.txt (the prediction a colleague made before the day was run, from those two files and the glossary), brief_final.txt (the final briefing, after the day), and glossary.md (the game's glossary). Read those files with the Read tool and no other file.
>
> You may also ask the game questions, as your side, with this one command and nothing else (Bash):
>   [scratch]/ct_leg/ask.sh TOOL 'ARGUMENTS_JSON'
> TOOL is one of: explain (arguments {"event": N}, an event number shown in a briefing), get_briefing, list_forces, list_contacts, list_bases, get_entity (arguments {"handle": "ship:1"} and the like), measure. Example: ask.sh explain '{"event": 22}'. You cannot change anything. Do not run any other program, open any other file, or use the web.
>
> The day is over. Using the final briefing, `explain` and the query tools, answer:
> 1. Pick the most important thing that happened to your side (a ship hit, a strike that did or did not fly, a loss). Explain step by step why it happened, citing event numbers and the chances or figures `explain` gives.
> 2. Your people claim certain enemy losses. What do you actually know of the enemy's losses, and why might the claim be wrong?
> 3. Where did the prediction go wrong, and what in the briefing should have told you?
> 4. If the game went on one more day, what would you order and what do you expect it to do?
> 5. Name anything you could not understand.
>
> Your final report must be your whole answer, exactly as you would hand it in, followed by a list of every ask.sh call you made.

`prediction.txt` was session 1's answer above, word for word. `ask.sh` refused every tool but the seven named, and ran each call on a copy of the finished game, so nothing asked was kept (`sim-cli play --game COPY TOOL ARGS` on the release build). Its log shows the 18 calls the answer lists.

### Answer

CHIEF OF STAFF REPORT, Eastern Solomons, 24 August 1942 (Allied, game over)

Result, from the final briefing: Allied 12.85 points, margin +12.55, major victory. Japanese 0.3 points. Allied points were 10 for one carrier sunk plus 2.85 for 57 Japanese aircraft destroyed (57 x 0.05). Japanese points were 0.3, from 6 Allied aircraft (6 x 0.05). No Allied ship was crippled.

1. The most important event: the afternoon strike on contact:1 and the carrier sunk.
- Search: air_group:3 (VS-5, Enterprise) searched 270-000 out to 200 nm with 16 aircraft (event 20). It sighted a force (event 21), reported as contact:1, 4 ships, taken for carrier, at (7,-9) (event 22). The sighting chance is not shown: event 21's breakdown lists aircraft 16, sweep width 20, arc 90, with no result. I did not check whether VS-3 or VMSB-232 also searched in the morning.
- Strike: the four naval_strike groups were set before the contact existed. The staff launched them in the afternoon pulse, because the contact was reported in that pulse at 160 nm, inside the 200 and 190 nm radii (events 25-28): VB-6 17 aircraft, VT-3 7, VB-3 15, VT-8 8, 47 in all. The default target list put carriers first.
- Attack: VB-6 16 bombs (event 39), VT-3 7 torpedoes (40), VB-3 15 bombs (41), VT-8 8 torpedoes (42), 46 shots. Losses on the strike: one VB-6 (event 29) and one VB-3 (event 46). The breakdowns show only aircraft counts, so I cannot cite the hit chances.
- Claims: 8 hits on a carrier, 0 sunk (event 60), then 6 hits, 1 sunk (event 61). The truth is one carrier sunk. The sinking event is not visible to my side, so I cannot say which group or which hit did it.
- Why it mattered: 10 points for the carrier, plus the aircraft points, took the margin past 8. Without the sinking the margin is about 2.5, a stalemate.
- Also in the afternoon, Japanese aircraft attacked Task Force 16 (event 43, 14 aircraft). Its cause is hidden, so I cannot explain their attack. Enterprise took 3 flotation hits, 15.78 points (event 56), 2 fire hits, 10.52 points (event 57), and her flight deck went from working to degraded (event 58). Main battery was unchanged (event 59). She is well short of the 50 that makes a ship crippled. The briefing says 21 aircraft in 2 attacks, against 14 in event 43; I could not reconcile those. Henderson was raided by 6 aircraft with 0 hits (event 45).

2. What we know of enemy losses, and why the claims may be wrong.
- The final result gives one carrier sunk, zero crippled, and 57 aircraft destroyed. That is the only check on the claims.
- The sinking claim matches the count. The carrier hits (14 claimed) cannot be checked against anything. The aircraft claim is 78 (44 + 25 over the fleet, 9 over Henderson), 21 above the 57 that happened.
- Why claims are high: the briefing says claims are never below the truth and rise with the fire fired. Here that was 123 + 123 anti-aircraft shots (events 48, 50), plus 67 patrol shots. Claims are never corrected.
- The "carrier" label is the kind at launch. The contact was later re-taken for a destroyer and then a battleship (events 54, 55), so the label may not match what was hit.
- The contact's count fell 5, 4, 3, 2 through the attacks (events 52-55). Those are reports, not facts, and the drop cannot show what sank.
- The history note says Ryujo was sunk and Enterprise withdrew for repair. The game has no withdrawal for Enterprise, and I cannot tie the game's sunk carrier to a name.

3. Where the prediction went wrong, and what the briefing should have said.
- Timing: it expected the sighting in the morning. Every search and strike event I read was in the afternoon. It gave no basis for the morning timing.
- Most at risk: it named Saratoga. Enterprise was the one hit, 5 hits and a flight deck down.
- Outcome: it called a stalemate, reasoning that no ship was likely sunk. It did write that a sunk carrier gives 10 points and a major victory, but it put low odds on the sinking. The briefing gave no sighting odds, so that estimate had nothing under it.
- Japanese air: it could not say whether one would come. One did, and it hit Enterprise.
- What the briefing should have told me: the strike is set in the same pulse as the sighting, so the strike can be ordered the day before, and a single carrier kill decides the level (10 + 2.85 - 0.3 = 12.55). Enterprise's position at (7,-5) with no move order left her exposed to the Japanese attack.

4. If the game went on one more day (hypothetical; the game is over).
- Keep the four strike groups on naval_strike with the default target list. Contact:1 was held at (6,-8), age 0, taken for battleship, at the end of the day, so a strike is likely in the morning pulse if it is still held. Expect the strike to be smaller, because Enterprise's flight deck is degraded (the glossary says degraded halves what the system does). I did not check how that applies to launches.
- Keep VS-3, VS-5 and VMSB-232 on search, since the remaining Japanese force is unlocated beyond contact:1.
- Raise the fighter patrol to full over Task Force 16 (half of VF-5 is on patrol). Expect another Japanese air attack on the carriers if their carriers are still afloat, and expect fewer Allied losses from it than from the first. Low confidence on all of this.
- I did not test any move or route order for the task forces, and the briefing does not say they exist.

5. What I could not understand.
- Sighting chances and hit chances: the breakdowns show factors but no result, so I cannot quote them.
- Which group, and which hit, sank the carrier.
- The Japanese attack's composition and causes (event 43 is cut).
- Whether the 14 claimed carrier hits were on the carrier that sank.
- The 21 versus 14 aircraft count on Enterprise.
- Whether VT-3 (air_group:4 and air_group:11) and VT-8 (air_group:8 and air_group:12) are double-counted; I did not test it.
- Whether Henderson's fuel of 0 t grounds VMF-223 and VMSB-232; the briefing says base supply is not in play, which may be the answer.
- Whether a task force mission does anything; the glossary says no rule reads it.

ask.sh calls made (18):
1. explain {"event": 22}
2. explain {"event": 25}
3. explain {"event": 39}
4. explain {"event": 40}
5. explain {"event": 41}
6. explain {"event": 42}
7. explain {"event": 56}
8. explain {"event": 57}
9. explain {"event": 58}
10. explain {"event": 59}
11. explain {"event": 60}
12. explain {"event": 61}
13. explain {"event": 62}
14. explain {"event": 48}
15. explain {"event": 32}
16. explain {"event": 45}
17. list_contacts {}
18. get_entity {"handle": "contact:1"}
