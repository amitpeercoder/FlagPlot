# Legibility test: the game the model was shown

The Allied side's view of one game of `data/scenarios/05_eastern_solomons.toml`, seed 1942, played with `sim-cli play` on commit b54840b (`main` after pull request 228). The record is `../closing-tests.md`. The orders are the two `issue_orders` calls of the recorded Sonnet game (`crates/sim-cli/tests/recorded_games/eastern_solomons_sonnet/game.jsonl`), copied as they stand. Session 1 was given the first two parts and the glossary; session 2 the opening briefing, the orders, the final briefing, the glossary, session 1's answer and the side's query tools. Neither was given the answer of `end_day`, which is here for the record.

## The opening briefing (`get_briefing` before the orders)

```
FLAG PLOT BRIEFING

HEADING
- Side: allied
- Day: 1 of 1
- This is the last day: the game ends when you end it.
- Scenario: Scenario 5: Eastern Solomons, 24 August 1942
- Covers: the state only, no day has been resolved

OBJECTIVES
- Mission: Find and sink the Japanese carriers supporting the reinforcement of Guadalcanal, and keep your own carriers in action. The Japanese carriers are expected to come down from the north.
- Objective: enemy ships sunk, points for each: carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2, auxiliary 1
  - What you know of your progress: nothing you can see shows it; it is counted at the end of the game
- Objective: enemy ships left afloat with propulsion or flight deck out, points for each: carrier 5, battleship 4, cruiser 2, destroyer 0.5, transport 1, auxiliary 0.5
  - What you know of your progress: nothing you can see shows it; it is counted at the end of the game
- Objective: enemy aircraft destroyed, points for each: 0.05
  - What you know of your progress: nothing you can see shows it; it is counted at the end of the game
- Levels: the result is read from the margin, your points less the enemy's. A margin of 3 or more is a minor victory, 8 or more a major one, 16 or more a decisive one; the same margins against you are defeats; less than 3 either way is a stalemate

ORDERS
- Accepted yesterday: 0
- Refused yesterday: none

CONTACTS
- New contacts: none
- Refreshed contacts: none
- Dropped contacts: none
- Contacts found not there: none
- Contacts held: none

ACTION YESTERDAY
- Fights at sea: none
- Salvoes fired at sea: none
- Own fighters' fire: none
- Own anti-aircraft fire: none
- Strikes and attacks flown: none
- Raids on bases: none
- Attacks on ships: none

LOSSES YESTERDAY
- Ships sunk: none
- Ships damaged: none
- Aircraft lost: none
- Claims yesterday: none

TASK FORCES
- task_force:1 "Task Force 16" at (7,-5): idle, carrier, speed cruise, readiness normal, not alert, 10 ships afloat
  - Least fuelled: ship:9 "USS Grayson" with 467 t, range 4730 miles at cruising speed
- task_force:2 "Task Force 11" at (7,-5): idle, carrier, speed cruise, readiness normal, not alert, 8 ships afloat
  - Least fuelled: ship:14 "USS Farragut" with 607 t, range 5240 miles at cruising speed

AIR GROUPS
- air_group:1 "VF-6" (Grumman F4F-4 Wildcat) at ship:1 "USS Enterprise" (7,-5): 27 ready, radius 105 miles, search range 105 miles, mission rest
- air_group:2 "VB-6" (Douglas SBD-3 Dauntless) at ship:1 "USS Enterprise" (7,-5): 17 ready, radius 200 miles, search range 397 miles, mission rest
- air_group:3 "VS-5" (Douglas SBD-3 Dauntless) at ship:1 "USS Enterprise" (7,-5): 16 ready, radius 200 miles, search range 397 miles, mission rest
- air_group:4 "VT-3" (Grumman TBF-1 Avenger) at ship:1 "USS Enterprise" (7,-5): 7 ready, radius 190 miles, search range 420 miles, mission rest
- air_group:5 "VF-5" (Grumman F4F-4 Wildcat) at ship:11 "USS Saratoga" (7,-5): 27 ready, radius 105 miles, search range 105 miles, mission rest
- air_group:6 "VB-3" (Douglas SBD-3 Dauntless) at ship:11 "USS Saratoga" (7,-5): 15 ready, radius 200 miles, search range 397 miles, mission rest
- air_group:7 "VS-3" (Douglas SBD-3 Dauntless) at ship:11 "USS Saratoga" (7,-5): 15 ready, radius 200 miles, search range 397 miles, mission rest
- air_group:8 "VT-8" (Grumman TBF-1 Avenger) at ship:11 "USS Saratoga" (7,-5): 8 ready, radius 190 miles, search range 420 miles, mission rest
- air_group:9 "VMF-223" (Grumman F4F-4 Wildcat) at base:1 "Henderson Field" (2,-4): 13 ready, radius 105 miles, search range 105 miles, mission rest
- air_group:10 "VMSB-232" (Douglas SBD-3 Dauntless) at base:1 "Henderson Field" (2,-4): 11 ready, radius 200 miles, search range 397 miles, mission rest
- air_group:11 "VT-3, attack group" (Grumman TBF-1 Avenger) at ship:1 "USS Enterprise" (7,-5): 7 ready, radius 190 miles, search range 420 miles, mission rest
- air_group:12 "VT-8, second group" (Grumman TBF-1 Avenger) at ship:11 "USS Saratoga" (7,-5): 5 ready, radius 190 miles, search range 420 miles, mission rest

BASES
- base:1 "Henderson Field" at (2,-4): fuel 0 t

ARRIVALS
- Arrivals due on the coming day and the next two days: none
- Replacements: none

WARNINGS
- Ships that cannot reach a base: none
- Bases short of supply: base supply is not in play
- Bases without fuel and a force stopped there: none
- Air groups short of pilots: none
- Air groups tired: fatigue is not in play
- Task forces that cannot move or are slowed: none
- Strike groups with nothing in reach: none
```

## The orders and the game's answers

```
### issue_orders call 1
{"orders": [{"order": {"from_deg": 0, "kind": "set_search_arc", "range_nm": "9999", "to_deg": 0}, "unit": "air_group:3"}], "reason": "probe max search range"}
### answer
{
  "cleared": false,
  "for_day": 1,
  "format": "flagplot-agent",
  "kind": "issue_orders",
  "queued": 0,
  "queued_in_call": 0,
  "refused_in_call": 1,
  "results": [
    {
      "index": 0,
      "reason": "search range 9999 nm is not above 0 and at most the aircraft's 397 nm",
      "status": "refused"
    }
  ],
  "summary": "In this call 0 of 1 orders queued and 1 refused. A refused order is not queued and the others in the call stand. 0 orders are now queued for day 1.",
  "version": 3
}
### issue_orders call 2
{"orders": [{"order": {"from_deg": 270, "kind": "set_search_arc", "range_nm": "200", "to_deg": 0}, "unit": "air_group:3"}, {"order": {"kind": "set_air_mission", "mission": "search"}, "unit": "air_group:3"}, {"order": {"from_deg": 0, "kind": "set_search_arc", "range_nm": "200", "to_deg": 90}, "unit": "air_group:7"}, {"order": {"kind": "set_air_mission", "mission": "search"}, "unit": "air_group:7"}, {"order": {"from_deg": 300, "kind": "set_search_arc", "range_nm": "200", "to_deg": 60}, "unit": "air_group:10"}, {"order": {"kind": "set_air_mission", "mission": "search"}, "unit": "air_group:10"}, {"order": {"kind": "set_air_mission", "mission": "naval_strike"}, "unit": "air_group:2"}, {"order": {"kind": "set_air_mission", "mission": "naval_strike"}, "unit": "air_group:4"}, {"order": {"kind": "set_air_mission", "mission": "naval_strike"}, "unit": "air_group:6"}, {"order": {"kind": "set_air_mission", "mission": "naval_strike"}, "unit": "air_group:8"}, {"order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}, "unit": "air_group:1"}, {"order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}, "unit": "air_group:5"}, {"order": {"kind": "set_patrol_share", "share": "0.5"}, "unit": "air_group:5"}, {"order": {"kind": "set_air_mission", "mission": "combat_air_patrol"}, "unit": "air_group:9"}, {"order": {"kind": "set_readiness", "readiness": "battle"}, "unit": "task_force:1"}, {"order": {"kind": "set_readiness", "readiness": "battle"}, "unit": "task_force:2"}], "reason": "Two dive-bomber scout groups and the Henderson dive-bombers search the northern half out to the 200 nm maximum; the other bomber and torpedo groups are set to strike any contact the search finds; fighters defend the carriers (half of VF-5 escorts); carriers at battle readiness. Contacts cannot be seen before the strike mission must be set, so everything is set now."}
### answer
{
  "cleared": false,
  "for_day": 1,
  "format": "flagplot-agent",
  "kind": "issue_orders",
  "queued": 16,
  "queued_in_call": 16,
  "refused_in_call": 0,
  "results": [
    {
      "index": 0,
      "status": "queued"
    },
    {
      "index": 1,
      "status": "queued"
    },
    {
      "index": 2,
      "status": "queued"
    },
    {
      "index": 3,
      "status": "queued"
    },
    {
      "index": 4,
      "status": "queued"
    },
    {
      "index": 5,
      "status": "queued"
    },
    {
      "index": 6,
      "status": "queued"
    },
    {
      "index": 7,
      "status": "queued"
    },
    {
      "index": 8,
      "status": "queued"
    },
    {
      "index": 9,
      "status": "queued"
    },
    {
      "index": 10,
      "status": "queued"
    },
    {
      "index": 11,
      "status": "queued"
    },
    {
      "index": 12,
      "status": "queued"
    },
    {
      "index": 13,
      "status": "queued"
    },
    {
      "index": 14,
      "status": "queued"
    },
    {
      "index": 15,
      "status": "queued"
    }
  ],
  "summary": "In this call 16 of 16 orders queued and 0 refused. A refused order is not queued and the others in the call stand. 16 orders are now queued for day 1.",
  "version": 3
}
```

## The final briefing (`get_briefing` after `end_day`)

```
FLAG PLOT BRIEFING

HEADING
- Side: allied
- Day: 2 (the game is over: day 1 was the last)
- Scenario: Scenario 5: Eastern Solomons, 24 August 1942
- Covers: day 1 and the state at the start of day 2

OBJECTIVES
- Mission: Find and sink the Japanese carriers supporting the reinforcement of Guadalcanal, and keep your own carriers in action. The Japanese carriers are expected to come down from the north.
- Objective: enemy ships sunk, points for each: carrier 10, battleship 8, cruiser 4, destroyer 1, transport 2, auxiliary 1
  - What you know of your progress: 1 enemy ships claimed sunk, worth 10 points for the kinds you attacked (claims can be wrong)
- Objective: enemy ships left afloat with propulsion or flight deck out, points for each: carrier 5, battleship 4, cruiser 2, destroyer 0.5, transport 1, auxiliary 0.5
  - What you know of your progress: nothing you can see shows it; it is counted at the end of the game
- Objective: enemy aircraft destroyed, points for each: 0.05
  - What you know of your progress: 78 enemy aircraft claimed shot down or destroyed on the ground, worth 3.9 points (claims can be wrong)
- Levels: the result is read from the margin, your points less the enemy's. A margin of 3 or more is a minor victory, 8 or more a major one, 16 or more a decisive one; the same margins against you are defeats; less than 3 either way is a stalemate
- The game is over. The result, counted from the truth:
- Allied: 12.85 points, margin 12.55, major victory
  - ships sunk: count 1 (1 carrier), points 10
  - ships crippled: count 0, points 0
  - aircraft destroyed: count 57, points 2.85
- Japanese: 0.3 points, margin -12.55, major defeat
  - ships sunk: count 0, points 0
  - ships crippled: count 0, points 0
  - aircraft destroyed: count 6, points 0.3
- History (what happened in the real battle, not in this game): Ryujo was sunk and Enterprise was damaged and withdrew for repair; Japanese aircraft losses were several times the American. Usually judged a limited American victory. (The architect's reading; not yet scored from the order-of-battle facts.)

ORDERS
- Accepted yesterday: 16
- Refused yesterday: none

CONTACTS
- New contact: contact:1, 4 ships estimated, taken for carrier, at (7,-9) [event 22]
- Refreshed contact: contact:1, 5 ships estimated (was 4), taken for carrier, at (7,-9) [event 52]
- Refreshed contact: contact:1, 4 ships estimated (was 5), taken for carrier, at (6,-8) (was (7,-9)) [event 53]
- Refreshed contact: contact:1, 3 ships estimated (was 4), taken for destroyer (was carrier), at (7,-9) (was (6,-8)) [event 54]
- Refreshed contact: contact:1, 2 ships estimated (was 3), taken for battleship (was destroyer), at (6,-8) (was (7,-9)) [event 55]
- Dropped contacts: none
- Contacts found not there: none
- Held contact: contact:1 at (6,-8), 2 ships estimated, taken for battleship, age 0 pulses

ACTION YESTERDAY
- Fights at sea: none
- Salvoes fired at sea: none
- Patrol fire: air_group:1 "VF-6" at the enemy's escorts, 4 shots [event 30]
- Patrol fire: air_group:1 "VF-6" at the enemy's bombers, 11 shots [event 31]
- Patrol fire: air_group:1 "VF-6" at the enemy's escorts, 6 shots [event 33]
- Patrol fire: air_group:1 "VF-6" at the enemy's bombers, 6 shots [event 34]
- Patrol fire: air_group:9 "VMF-223" at the enemy's escorts, 9 shots [event 36]
- Patrol fire: air_group:9 "VMF-223" at the enemy's bombers, 1 shots [event 37]
- Patrol fire: air_group:1 "VF-6" at the enemy's bombers, 18 shots [event 47]
- Patrol fire: air_group:1 "VF-6" at the enemy's bombers, 9 shots [event 49]
- Patrol fire: air_group:9 "VMF-223" at the enemy's bombers, 3 shots [event 51]
- Anti-aircraft fire: task_force:1 "Task Force 16" at a raid, 123 shots [event 48]
- Anti-aircraft fire: task_force:1 "Task Force 16" at a raid, 123 shots [event 50]
- Naval strike launched: air_group:2 "VB-6" at contact:1, place (7,-9), 17 aircraft, 160 miles [event 25]
- Naval strike launched: air_group:4 "VT-3" at contact:1, place (7,-9), 7 aircraft, 160 miles [event 26]
- Naval strike launched: air_group:6 "VB-3" at contact:1, place (7,-9), 15 aircraft, 160 miles [event 27]
- Naval strike launched: air_group:8 "VT-8" at contact:1, place (7,-9), 8 aircraft, 160 miles [event 28]
- Air attack: air_group:2 "VB-6", 16 aircraft, bombs [event 39]
  - Report: refreshed contact contact:1, 5 ships estimated, taken for carrier, at (7,-9) [event 52]
- Air attack: air_group:4 "VT-3", 7 aircraft, torpedoes [event 40]
  - Report: refreshed contact contact:1, 4 ships estimated, taken for carrier, at (6,-8) [event 53]
- Air attack: air_group:6 "VB-3", 15 aircraft, bombs [event 41]
  - Report: refreshed contact contact:1, 3 ships estimated, taken for destroyer, at (7,-9) [event 54]
- Air attack: air_group:8 "VT-8", 8 aircraft, torpedoes [event 42]
  - Report: refreshed contact contact:1, 2 ships estimated, taken for battleship, at (6,-8) [event 55]
- Raid on base:1 "Henderson Field": 6 aircraft attacked, 0 hits, 0 on aircraft, 0 on supply, 0 on nothing [event 45]
- Attack on ship:1 "USS Enterprise" from the air: 21 aircraft, 2 attacks

LOSSES YESTERDAY
- Ships sunk: none
- Ship damaged: ship:1 "USS Enterprise": 5 hits, flotation damage 15.782826, fire damage 10.521884, flight deck working to degraded, main battery working to working [events 56, 57, 58, 59]
- Aircraft lost: air_group:1 "VF-6": 2 shot down [events 32, 35]
- Aircraft lost: air_group:2 "VB-6": 1 shot down [event 29]
- Aircraft lost: air_group:6 "VB-3": 1 shot down [event 46]
- Aircraft lost: air_group:9 "VMF-223": 2 shot down [event 38]
- Claim at (7,-9): 8 hits on a carrier, 0 sunk [event 60]
- Claim at (7,-9): 6 hits on a carrier, 1 sunk [event 61]
- Claim over (7,-5): 44 aircraft shot down [event 62]
- Claim over (7,-5): 25 aircraft shot down [event 63]
- Claim over (2,-4) Henderson Field: 9 aircraft shot down [event 64]

TASK FORCES
- task_force:1 "Task Force 16" at (7,-5): idle, carrier, speed cruise, readiness battle, alert, 10 ships afloat
  - Least fuelled: ship:9 "USS Grayson" with 467 t, range 4730 miles at cruising speed
- task_force:2 "Task Force 11" at (7,-5): idle, carrier, speed cruise, readiness battle, alert, 8 ships afloat
  - Least fuelled: ship:14 "USS Farragut" with 607 t, range 5240 miles at cruising speed

AIR GROUPS
- air_group:1 "VF-6" (Grumman F4F-4 Wildcat) at ship:1 "USS Enterprise" (7,-5): 25 ready, radius 105 miles, search range 105 miles, mission combat air patrol
- air_group:2 "VB-6" (Douglas SBD-3 Dauntless) at ship:1 "USS Enterprise" (7,-5): 16 ready, radius 200 miles, search range 397 miles, mission naval strike, would strike contact:1 now (battleship, 144 miles)
- air_group:3 "VS-5" (Douglas SBD-3 Dauntless) at ship:1 "USS Enterprise" (7,-5): 16 ready, radius 200 miles, search range 397 miles, mission search
- air_group:4 "VT-3" (Grumman TBF-1 Avenger) at ship:1 "USS Enterprise" (7,-5): 7 ready, radius 190 miles, search range 420 miles, mission naval strike, would strike contact:1 now (battleship, 144 miles)
- air_group:5 "VF-5" (Grumman F4F-4 Wildcat) at ship:11 "USS Saratoga" (7,-5): 27 ready, radius 105 miles, search range 105 miles, mission combat air patrol
- air_group:6 "VB-3" (Douglas SBD-3 Dauntless) at ship:11 "USS Saratoga" (7,-5): 14 ready, radius 200 miles, search range 397 miles, mission naval strike, would strike contact:1 now (battleship, 144 miles)
- air_group:7 "VS-3" (Douglas SBD-3 Dauntless) at ship:11 "USS Saratoga" (7,-5): 15 ready, radius 200 miles, search range 397 miles, mission search
- air_group:8 "VT-8" (Grumman TBF-1 Avenger) at ship:11 "USS Saratoga" (7,-5): 8 ready, radius 190 miles, search range 420 miles, mission naval strike, would strike contact:1 now (battleship, 144 miles)
- air_group:9 "VMF-223" (Grumman F4F-4 Wildcat) at base:1 "Henderson Field" (2,-4): 11 ready, radius 105 miles, search range 105 miles, mission combat air patrol
- air_group:10 "VMSB-232" (Douglas SBD-3 Dauntless) at base:1 "Henderson Field" (2,-4): 11 ready, radius 200 miles, search range 397 miles, mission search
- air_group:11 "VT-3, attack group" (Grumman TBF-1 Avenger) at ship:1 "USS Enterprise" (7,-5): 7 ready, radius 190 miles, search range 420 miles, mission rest
- air_group:12 "VT-8, second group" (Grumman TBF-1 Avenger) at ship:11 "USS Saratoga" (7,-5): 5 ready, radius 190 miles, search range 420 miles, mission rest

BASES
- base:1 "Henderson Field" at (2,-4): fuel 0 t

ARRIVALS
- Arrivals due on the coming day and the next two days: none
- Replacements: none

WARNINGS
- Ships that cannot reach a base: none
- Bases short of supply: base supply is not in play
- Bases without fuel and a force stopped there: none
- Air groups short of pilots: none
- Air groups tired: fatigue is not in play
- Task forces that cannot move or are slowed: none
- Strike groups with nothing in reach: none
```

## The answer of `end_day` (not shown to either session)

```
{
  "day": 1,
  "format": "flagplot-agent",
  "game_over": true,
  "kind": "end_day",
  "resolved": true,
  "result": {
    "allied": {
      "level": "major",
      "margin": "12.55",
      "objectives": [
        {
          "base": null,
          "closed": null,
          "count": "1",
          "days": null,
          "kind": "ships_sunk",
          "points": "10",
          "ships": [
            {
              "kind": "carrier",
              "ships": 1
            }
          ]
        },
        {
          "base": null,
          "closed": null,
          "count": "0",
          "days": null,
          "kind": "ships_crippled",
          "points": "0",
          "ships": []
        },
        {
          "base": null,
          "closed": null,
          "count": "57",
          "days": null,
          "kind": "aircraft_destroyed",
          "points": "2.85",
          "ships": []
        }
      ],
      "outcome": "major victory",
      "result": "victory",
      "total": "12.85"
    },
    "history": "Ryujo was sunk and Enterprise was damaged and withdrew for repair; Japanese aircraft losses were several times the American. Usually judged a limited American victory. (The architect's reading; not yet scored from the order-of-battle facts.)",
    "japanese": {
      "level": "major",
      "margin": "-12.55",
      "objectives": [
        {
          "base": null,
          "closed": null,
          "count": "0",
          "days": null,
          "kind": "ships_sunk",
          "points": "0",
          "ships": []
        },
        {
          "base": null,
          "closed": null,
          "count": "0",
          "days": null,
          "kind": "ships_crippled",
          "points": "0",
          "ships": []
        },
        {
          "base": null,
          "closed": null,
          "count": "6",
          "days": null,
          "kind": "aircraft_destroyed",
          "points": "0.3",
          "ships": []
        }
      ],
      "outcome": "major defeat",
      "result": "defeat",
      "total": "0.3"
    }
  },
  "version": 3
}
```
