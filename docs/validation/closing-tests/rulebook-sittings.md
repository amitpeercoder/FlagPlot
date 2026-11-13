# Rulebook-only test: the sittings, the files and the prompts

Four sittings, each a fresh subagent on the model `haiku` (Claude Haiku), run in parallel on Nov 13, 2026, on commit b54840b (`main` after pull request 228). The record and the marking are `../closing-tests.md`; the answers, word for word, are `rulebook-answers-1.md` to `rulebook-answers-4.md`.

## What each sitting was given

A folder holding nothing else:

- `rules/`: every file of `docs/rules/` at the tested commit (README, chapters 1 to 6, differences).
- `glossary.md`: `docs/glossary.md` at the tested commit.
- `scenarios/`: the two scenario files, with their comment lines taken out (they state some of the answers) and their reference paths pointed at the copies below. Nothing else in them was changed.
- `reference-<scenario>/`: the frozen figures (`data/reference/step1`, every file but the README) as the scenario's test reads them, that is with the test's own alterations made by the same exact, unique replacements as `common::altered_reference` (script below). Where a question sets a figure otherwise (a share b, s or p, or a changed count of aircraft or ships), the question says so in words, as the test does by editing the text.
- `map/`: the map file each scenario names.

Not given: the tests, the decisions, the code, the specs, run baselines. The model was told to open no file outside the folder and run no program; each answer ends with the list of files it read, all inside its folder.

The copies were checked by running them: scenario 16's copy on its altered figures gives the pinned outcome of `tests/scenario_16_all_together.rs` section 7 (the field at 490 t, Allied 10.5 and Japanese 16.5 points); scenario 17a's case 1 text gives the salvo targets of the test's case 1 table; every other copy loads and runs.

| Sitting | Scenario files | Figures, as the test alters them | Rule changes covered | Hand calculation the expected numbers come from |
| --- | --- | --- | --- | --- |
| 1 | `16_all_together.toml`, `15b_unready_torpedoes.toml` | 16: the twelve rows of its table (experience strengths 0.4, `unready_gun_share` 0.5, `system_hit_chance_per_point` 0.1, bombs at 1 and on the deck, crew recovery, no wear, true reports); 15b: `unready_gun_share` and `unready_torpedo_share` 0.5 | 0058, 0064, 0068, 0071 | `tests/scenario_16_all_together.rs` sections 1 and 2; `tests/scenario_15b_unready_torpedoes.rs`, with `tests/scenario_15a_unready_fire.rs` for the gun shots |
| 2 | `17a_target_choice.toml` (case 1, written out from the test's `scenario_text`), `18_torpedo_sides.toml` | 17a: none; 18: both unready shares 0.5 | 0078, 0083 | `tests/scenario_17a_target_choice.rs` cases 1 and 7 and the points table; `tests/scenario_18_torpedo_sides.rs` cases A, A', C, D |
| 3 | `19a_guns_and_the_attack.toml`, `19b_whom_the_patrol_fires_at.toml` | `strike_contact_max_age_pulses` 1 | 0079, 0080 | `tests/scenario_19a_guns_and_the_attack.rs` cases 1 and 2; `tests/scenario_19b_whom_the_patrol_fires_at.rs` cases (a), (b), (d), (f) |
| 4 | `19c_patrol_after_the_attack.toml`, `04a_night_action.toml` | 19c: `strike_contact_max_age_pulses` 1; 04a: none | 0087 (with 0079 and 0080); step 1 as the control | `tests/scenario_19c_patrol_after_the_attack.rs` cases 1, 2 and 4; `tests/scenario_04a_night_action.rs` (the meeting, who sights whom, a salvo) |

## Sitting 1: the prompt, word for word

Followed in the call by one more line: "Your final report must be your whole worked answer, every step, exactly as you would hand it in." [scratch] stands for the session's scratch folder.

```text
You are given a game's rulebook, its glossary, its data files and two scenario files, all in the folder [scratch]/ct_rulebook/sitting1 and nothing outside it:
- rules/ : the rulebook, every chapter (start with rules/README.md)
- glossary.md : the glossary
- scenarios/ : the two scenario files
- reference-*/ and map/ : the data files each scenario names (its paths are relative to the scenarios/ folder)

Read only files inside that folder, with the Read tool (Glob/Grep inside the folder are fine). Do not open any file outside it, do not run any program, and do not use the web. At the end, list every file you read.

The game is deterministic except for random draws; where a draw decides something, give the chance, not the outcome. Using only these files, work the following by hand. Show every step: the rule you use (chapter and section), each figure you read (file and key), and each intermediate number. Numbers are fixed-point with six decimals: give every result to six decimals and say where you rounded and how (cut or round). Where the book does not tell you something you need, say so, say what you assumed, and go on.

Scenario files: scenarios/16_all_together.toml (figures in reference-16/) and scenarios/15b_unready_torpedoes.toml (figures in reference-15b/).

Questions on scenario 16 (the night pulse of day 1 only):
1. The Japanese cruiser force (task force 4) steams toward (2,-4), where the Allied cruiser force (task force 3) lies. After how many miles, and at what time, does it enter (2,-4) and the fight begin?
2. When the fight begins, is each force alert? Is each force ready? Does either change during the fight, and does that change what it fires?
3. Each force's experience factor, and each force's chance to sight the other in round 1.
4. Gun shots a round fired by one Japanese cruiser and by one Allied cruiser, each with its main battery working; and by an Allied cruiser whose main battery is degraded.
5. The chance that one gun shot hits, and the chance that one torpedo hits, for a Japanese firer and for an Allied firer, against a target that is (a) neither surprised nor lit, (b) surprised, (c) lit (guns only). Then the gun chance of an Allied firer whose fire control is degraded, against a target neither surprised nor lit.
6. The expected hits of one Allied gun salvo and of one Japanese gun salvo, each against a target neither surprised nor lit; and the torpedo shots each cruiser of each side fires.

Questions on scenario 15b (the night fight of task forces 1 and 2):
7. Is force B (task force 2) ready when the fight begins? How many gun shots does one of its cruisers fire in a round with its main battery working, and how many hits are expected against a target neither surprised nor lit?
8. How many torpedo shots does one of force B's cruisers fire, and how many hits are expected (target neither surprised nor lit), (a) with its torpedo mounts sound, (b) with its mounts degraded? After that salvo, can she fire her remaining tubes later?
9. The same cruiser if force B had been ordered to battle readiness: torpedo shots and expected hits at night.
10. Suppose instead the fight were by day, in the morning pulse. (a) Force B had been alerted in the night pulse, before the pulse of the fight: torpedo shots and expected hits of one of its cruisers. (b) Force B had not been alerted at all: the same.

End with a short list headed "Where I had to assume or infer", one line each, quoting the rulebook line that was unclear where there is one.
```

## Sitting 2: the prompt, word for word

Followed in the call by one more line: "Your final report must be your whole worked answer, every step, exactly as you would hand it in." [scratch] stands for the session's scratch folder.

```text
You are given a game's rulebook, its glossary, its data files and two scenario files, all in the folder [scratch]/ct_rulebook/sitting2 and nothing outside it:
- rules/ : the rulebook, every chapter (start with rules/README.md)
- glossary.md : the glossary
- scenarios/ : the two scenario files
- reference-*/ and map/ : the data files each scenario names (its paths are relative to the scenarios/ folder)

Read only files inside that folder, with the Read tool (Glob/Grep inside the folder are fine). Do not open any file outside it, do not run any program, and do not use the web. At the end, list every file you read.

The game is deterministic except for random draws; where a draw decides something, give the chance, not the outcome. Using only these files, work the following by hand. Show every step: the rule you use (chapter and section), each figure you read (file and key), and each intermediate number. Numbers are fixed-point with six decimals: give every result to six decimals and say where you rounded and how (cut or round). Where the book does not tell you something you need, say so, say what you assumed, and go on.

Scenario files: scenarios/17a_target_choice.toml (figures in reference-17a/) and scenarios/18_torpedo_sides.toml (figures in reference-18/).

Questions on scenario 17a (one fight, by day, in the morning pulse; ships 1 to 4 are force X, ships 5 to 8 force Y):
1. In round 1, at which enemy ship does each of the eight ships fire its guns, and at which its torpedoes? Give the reason for each.
2. The shots and expected hits of one gun salvo of a cruiser, of one gun salvo of a destroyer, and of one torpedo salvo, in round 1.
3. The points of damage one hit does: a cruiser's gun, a destroyer's gun and a torpedo, each on a cruiser (class 1), a destroyer (class 2) and a cargo ship (class 3). Six decimals.
4. The same file with force X changed to ships 1 a cargo ship (class 3), 2 and 3 cruisers (class 1), and force Y to ships 4, 5 and 6, all cruisers (class 1). In round 1, at which ship does each ship fire its guns and its torpedoes?

Questions on scenario 18 (the night fight of task forces 1 and 2; the ships of force A, task force 1, are class 8):
5. A ship of force A, in a night fight of two rounds or more, target not surprised: in round 1 and in round 2, how many tubes fire, how many torpedo shots, the chance a shot hits, and the expected hits? Is anything left to fire afterwards?
6. As 5, but the target's force is surprised in round 1: round 1's chance and expected hits.
7. If force A were not ready when the fight began: round 1 and round 2 tubes, shots, chance and expected hits.
8. If her torpedo mounts were degraded before round 1 (force A ready): round 1 and round 2 tubes, shots and expected hits.
9. The points of damage a torpedo hit from her does to a class 1 cruiser, and the chance that hit knocks a system down a level.

End with a short list headed "Where I had to assume or infer", one line each, quoting the rulebook line that was unclear where there is one.
```

## Sitting 3: the prompt, word for word

Followed in the call by one more line: "Your final report must be your whole worked answer, every step, exactly as you would hand it in." [scratch] stands for the session's scratch folder.

```text
You are given a game's rulebook, its glossary, its data files and two scenario files, all in the folder [scratch]/ct_rulebook/sitting3 and nothing outside it:
- rules/ : the rulebook, every chapter (start with rules/README.md)
- glossary.md : the glossary
- scenarios/ : the two scenario files
- reference-*/ and map/ : the data files each scenario names (its paths are relative to the scenarios/ folder)

Read only files inside that folder, with the Read tool (Glob/Grep inside the folder are fine). Do not open any file outside it, do not run any program, and do not use the web. At the end, list every file you read.

The game is deterministic except for random draws; where a draw decides something, give the chance, not the outcome. Using only these files, work the following by hand. Show every step: the rule you use (chapter and section), each figure you read (file and key), and each intermediate number. Numbers are fixed-point with six decimals: give every result to six decimals and say where you rounded and how (cut or round). Where the book does not tell you something you need, say so, say what you assumed, and go on.

Scenario files: scenarios/19a_guns_and_the_attack.toml (figures in reference-19a/) and scenarios/19b_whom_the_patrol_fires_at.toml (figures in reference-19b/).

Questions on scenario 19a (the morning pulse of day 1). Call b the figure anti_aircraft_before_attack_share in air_combat.toml; it is 1 in the file given, and each question says where it is set otherwise.
1. Which groups launch, how many aircraft, and at what? Which aircraft go as escorts and why?
2. Over task force 1: the shots, chance and expected hits of (a) the escorts at the patrol, (b) the patrol at the escorts, (c) the patrol at the bombers, (d) the anti-aircraft guns in all.
3. For b = 1, 0.5, 0.33 and 0: the guns' shots before the attack and after it; the expected number of bombers that attack (attack shots); the expected bomb hits on the carrier; the expected bombers lost to the guns.
4. The points of damage one bomb hit does to the carrier.
5. The same file with group 2 at 1 dive bomber, group 3 at 0 and group 1 at 0. For b = 1, 0.5 and 0: the chance the bomber attacks, its expected bomb hits, and the chance it is lost.

Questions on scenario 19b (the morning pulse of day 1). Call s the figure patrol_drawn_off_share in air_combat.toml; it is 1 in the file given, and each question says where it is set otherwise.
6. As the file stands (patrol 4, escorts 6, bombers 10), for s = 1, 0.5 and 0: how many patrol shots go at the escorts and how many at the bombers, with the expected hits of each, and the escorts' shots at the patrol with their expected hits.
7. The same with group 1 at 10, group 2 at 12 and group 3 at 4, for s = 1, 0.5 and 0.
8. The same with group 1 at 20, group 2 at 3 and group 3 at 2, for s = 1 and 0.
9. Task force 2 has two carriers, each with its own dive bombers on naval strike and its own fighters escorting (patrol share 0): the first carrier's raid has 8 bombers and 2 escorts, the second's 6 bombers and 4 escorts; both strike task force 1, whose patrol is 10. For s = 1, 0.5 and 0: each raid's share of the patrol, and the patrol's shots at each raid's escorts and bombers.

End with a short list headed "Where I had to assume or infer", one line each, quoting the rulebook line that was unclear where there is one.
```

## Sitting 4: the prompt, word for word

Followed in the call by one more line: "Your final report must be your whole worked answer, every step, exactly as you would hand it in." [scratch] stands for the session's scratch folder.

```text
You are given a game's rulebook, its glossary, its data files and two scenario files, all in the folder [scratch]/ct_rulebook/sitting4 and nothing outside it:
- rules/ : the rulebook, every chapter (start with rules/README.md)
- glossary.md : the glossary
- scenarios/ : the two scenario files
- reference-*/ and map/ : the data files each scenario names (its paths are relative to the scenarios/ folder)

Read only files inside that folder, with the Read tool (Glob/Grep inside the folder are fine). Do not open any file outside it, do not run any program, and do not use the web. At the end, list every file you read.

The game is deterministic except for random draws; where a draw decides something, give the chance, not the outcome. Using only these files, work the following by hand. Show every step: the rule you use (chapter and section), each figure you read (file and key), and each intermediate number. Numbers are fixed-point with six decimals: give every result to six decimals and say where you rounded and how (cut or round). Where the book does not tell you something you need, say so, say what you assumed, and go on.

Scenario files: scenarios/19c_patrol_after_the_attack.toml (figures in reference-19c/) and scenarios/04a_night_action.toml (figures in reference-04a/).

Questions on scenario 19c (the morning pulse of day 1). Call p the figure patrol_before_attack_share and b the figure anti_aircraft_before_attack_share, both in air_combat.toml; both are 1 in the files given.
1. As the file stands: the patrol, escorts and bombers; the shots of the escorts at the patrol, of the patrol at the escorts and of the patrol at the bombers, with expected hits.
2. For p = 1, 0.4 and 0: the patrol's shots at the bombers before the attack and after it; the expected attack shots; the expected bomb hits; the expected bombers lost to the patrol.
3. The same file with group 2 at 1 dive bomber, group 3 at 0 and group 1 at 5. For p = 1, 0.4 and 0: the patrol's shots before and after; the chance the bomber attacks; its expected bomb hits; the chance it is lost.
4. The same file with task force 1's carrier of class 5, group 1 at 6 and group 3 at 0. For (p, b) = (1, 1), (0.4, 1), (0.4, 0) and (0, 0): the patrol's shots before and after, the guns' shots before and after, the expected attack shots and the expected bomb hits.

Questions on scenario 04a (the night pulse of day 1):
5. When, and after how many miles, do the forces meet, and how much fuel has each cruiser of force A burned by then?
6. Each force's chance to sight the other in a round; the chances, in a round where neither has yet sighted, that both sight, only A, only B, neither.
7. The chance that neither ever sights in the fight; and the chances that A has a free round, B has a free round, both fire from the start.
8. Shots, chance and expected hits of a cruiser's gun salvo against a target (a) not surprised, (b) surprised, (c) lit; with its fire control degraded (target not surprised); a torpedo salvo against a target not surprised and surprised.
9. The points one gun hit and one torpedo hit do to a placeholder cruiser, and the chance each knocks a system down a level.
10. The chance that a cruiser of A firing in a free round at a surprised target scores at least one torpedo hit.

End with a short list headed "Where I had to assume or infer", one line each, quoting the rulebook line that was unclear where there is one.
```

## Scenario 17a, case 1, as given

The text `scenario_text(&[1, 1, 2, 2], &[1, 2, 2, 2])` of `tests/scenario_17a_target_choice.rs` builds, written out:

```toml
name = "Scenario 17a: which enemy ship a ship fires at"
placeholder = true
seed = 1942
days = 1
map = "../map/placeholder_night_action.toml"
ship_classes = "../reference/ship_classes.csv"
port_sizes = "../reference/port_sizes.csv"
aircraft_types = "../reference/aircraft_types.csv"
search = "../reference/search.toml"
surface_combat = "../reference/surface_combat.toml"

[[task_force]]
id = 1
name = "Placeholder force X"
side = "japanese"
q = 0
r = 0
mission = "surface_combat"
readiness = "battle"

[[task_force]]
id = 2
name = "Placeholder force Y"
side = "allied"
q = 6
r = 0
mission = "surface_combat"
readiness = "battle"

[[ship]]
id = 1
name = "Placeholder cruiser 1"
class = 1
side = "japanese"
fuel_t = 2000
task_force = 1

[[ship]]
id = 2
name = "Placeholder cruiser 2"
class = 1
side = "japanese"
fuel_t = 2000
task_force = 1

[[ship]]
id = 3
name = "Placeholder destroyer 3"
class = 2
side = "japanese"
fuel_t = 500
task_force = 1

[[ship]]
id = 4
name = "Placeholder destroyer 4"
class = 2
side = "japanese"
fuel_t = 500
task_force = 1

[[ship]]
id = 5
name = "Placeholder cruiser 5"
class = 1
side = "allied"
fuel_t = 2000
task_force = 2

[[ship]]
id = 6
name = "Placeholder destroyer 6"
class = 2
side = "allied"
fuel_t = 500
task_force = 2

[[ship]]
id = 7
name = "Placeholder destroyer 7"
class = 2
side = "allied"
fuel_t = 500
task_force = 2

[[ship]]
id = 8
name = "Placeholder destroyer 8"
class = 2
side = "allied"
fuel_t = 500
task_force = 2

[[order]]
day = 1
side = "japanese"
task_force = 1
kind = "set_rules_of_engagement"
stance = "engage"
break_off = "1"

[[order]]
day = 1
side = "allied"
task_force = 2
kind = "set_route"
waypoints = [[0, 0]]

[[order]]
day = 1
side = "allied"
task_force = 2
kind = "set_speed"
speed = "cruise"

[[order]]
day = 1
side = "allied"
task_force = 2
kind = "set_rules_of_engagement"
stance = "engage"
break_off = "1"
```

## How the altered figures were made

The script, run once per scenario as `python3 -I make_figures.py REPO OUT VARIANT`:

```python
"""Make the altered copies of the frozen figures the hand-calculation tests use.

Mirrors tests/common/mod.rs altered_reference + edit_file_by (each replacement
exact and unique). Usage: python3 -I ct_make_figures.py REPO OUT_DIR VARIANT
"""
import os
import shutil
import sys

repo, out, variant = sys.argv[1], sys.argv[2], sys.argv[3]
frozen = os.path.join(repo, "data/reference/step1")


def replace_once(text, a, b):
    assert text.count(a) == 1, (a, text.count(a))
    return text.replace(a, b)


def edit(name, pairs):
    p = os.path.join(out, name)
    t = open(p).read()
    for a, b in pairs:
        t = replace_once(t, a, b)
    open(p, "w").write(t)


os.makedirs(out, exist_ok=True)
for f in os.listdir(frozen):
    if f.endswith(".md"):
        continue
    shutil.copy(os.path.join(frozen, f), os.path.join(out, f))

if variant == "16":
    edit("surface_combat.toml", [
        ('experience_gun_strength = "0"', 'experience_gun_strength = "0.4"'),
        ('experience_torpedo_strength = "0"', 'experience_torpedo_strength = "0.4"'),
        ('experience_sighting_strength = "0"', 'experience_sighting_strength = "0.4"'),
        ('unready_gun_share = "1"', 'unready_gun_share = "0.5"'),
        ('system_hit_chance_per_point = "0.06"', 'system_hit_chance_per_point = "0.1"'),
    ])
    edit("air_combat.toml", [
        ('bomb_hit_chance = "0.2"', 'bomb_hit_chance = "1"'),
        ('flight_deck = "0.5"', 'flight_deck = "1"'),
        ('propulsion = "0.2"', 'propulsion = "0"'),
        ('fire_control = "0.1"', 'fire_control = "0"'),
        ('main_battery = "0.2"', 'main_battery = "0"'),
        ('crew_recovered_beside_own_force = "1"', 'crew_recovered_beside_own_force = "0.95"'),
        ('crew_recovered_open_sea = "1"', 'crew_recovered_open_sea = "0.5"'),
        ('crew_recovered_over_own_base = "1"', 'crew_recovered_over_own_base = "0.6"'),
    ])
    edit("air_upkeep.toml", [
        ('wear_base_chance = "0.05"', 'wear_base_chance = "0"'),
        ('wear_chance_per_100_nm = "0.02"', 'wear_chance_per_100_nm = "0"'),
    ])
    p = os.path.join(out, "search.toml")
    t = open(p).read()
    at = t.find("[kind_confusion.carrier]")
    head = t[:at]
    for a, b in [
        ('position_error_chance = "0.3"', 'position_error_chance = "0"'),
        ('count_over_chance = "0.2"', 'count_over_chance = "0"'),
        ('count_under_chance = "0.2"', 'count_under_chance = "0"'),
    ]:
        head = replace_once(head, a, b)
    for kind in ["carrier", "battleship", "cruiser", "destroyer", "transport", "auxiliary"]:
        head += f'[kind_confusion.{kind}]\n{kind} = "1"\n\n'
    open(p, "w").write(head)
elif variant in ("15b", "18"):
    edit("surface_combat.toml", [
        ('unready_gun_share = "1"', 'unready_gun_share = "0.5"'),
        ('unready_torpedo_share = "1"', 'unready_torpedo_share = "0.5"'),
    ])
elif variant in ("19a", "19b", "19c"):
    edit("air_combat.toml", [
        ('strike_contact_max_age_pulses = 0', 'strike_contact_max_age_pulses = 1'),
    ])
elif variant in ("17a", "04a"):
    pass
else:
    raise SystemExit("unknown variant " + variant)
```
