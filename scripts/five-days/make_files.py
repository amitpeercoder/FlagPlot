"""Make the four files of scenario 103, the Eastern Solomons over five days
(decision 0184), from its base, or check that the files kept are the ones it
makes.

    python3 scripts/five-days/make_files.py [REPO] [--check]

REPO is the top of the repository (by default, two folders above this
script). It reads `data/scenarios/five-days/base_eastern_solomons.toml` as
text and writes `data/scenarios/five-days/01a_eastern_solomons.toml` to
`01d_eastern_solomons.toml`. With `--check` it writes nothing: it says
whether each file kept is byte for byte the one it would make, and exits 1
where one is not. Python 3.9 or later, standard library only,
deterministic. No build step runs it; the scenario's test runs it with
`--check`. Run it by hand whenever the base or this script changes.

Each file is the base with exactly these changes, and nothing else:
  1. the header below, the same in all four, put before the base;
  2. its `name`;
  3. at the end, the `[[command]]` tables of its pairing (decision 0185):
     both sides given their orders (a), the Japanese side alone (b), the
     Allied side alone (c), neither (d). The orders are the hand
     calculation's ("What a side the engine plays is given"), on day 1.
So with their names and `[[command]]` tables taken away the four are the
same, byte for byte.
"""
import argparse
import os
import sys

BASE = 'data/scenarios/five-days/base_eastern_solomons.toml'
FOLDER = 'data/scenarios/five-days'

FILES = [
    # (file, letter, Allied given orders, Japanese given orders)
    ('01a_eastern_solomons.toml', 'a', True, True),
    ('01b_eastern_solomons.toml', 'b', False, True),
    ('01c_eastern_solomons.toml', 'c', True, False),
    ('01d_eastern_solomons.toml', 'd', False, False),
]

NAME_OLD = ('name = "Scenario 103, the base: the Eastern Solomons over five days, '
            '23 to 27 August 1942"\n')

WORDS = {
    'a': 'both sides given their orders',
    'b': 'the Japanese side given its orders, the Allied side none',
    'c': 'the Allied side given its orders, the Japanese side none',
    'd': 'neither side given orders',
}

HEADER = '''# SCENARIO 103, THE EASTERN SOLOMONS OVER FIVE DAYS (decision 0184).
#
# Made by scripts/five-days/make_files.py from the base,
# data/scenarios/five-days/base_eastern_solomons.toml. Do not edit by hand:
# change the base or the script and make the files again
# (`python3 scripts/five-days/make_files.py --check` says whether each file is
# still the one it makes). There are four files, one for each pairing the
# table offers (decision 0185):
#   01a: both sides given their orders (staff against staff);
#   01b: the Japanese side given its orders, the Allied side none (a person
#        or an agent takes the Allied seat);
#   01c: the Allied side given its orders, the Japanese side none;
#   01d: neither side given orders.
# Each is the base with this header, a name of its own and, at the end, the
# `[[command]]` tables of the sides it gives orders, and nothing else: with
# their names and command tables taken away the four are the same, byte for
# byte. The orders are the hand calculation's (tests/
# scenario_103_the_eastern_solomons_over_five_days.rs, "What a side the
# engine plays is given"), on day 1, with no risk set (balanced):
#   Japanese: intent 1, hold Taivu (base 5), its forces the convoy (task
#     force 5) and its three land units (27, 28, 29), with Rabaul (base 2)
#     the source the convoy returns to; intent 2, neutralise Lunga (base 1),
#     its forces the land bombers and the fighters at Rabaul (groups 27, 28);
#     intent 3, operate task force 3 from (8,-11); intent 4, operate task
#     force 4 from (7,-9), each with its own air groups (no forces limit);
#     leave to prepare and leave to launch given.
#   Allied: intent 1, hold Lunga (base 1), its forces the four land units
#     there (1 to 4) and the two squadrons (groups 9 and 10); intent 2,
#     operate task force 1 from (6,-3); intent 3, operate task force 2 from
#     (7,-3), each with its own air groups (no forces limit); intent 4,
#     deny the waters within 2 hexes of (3,-7), with no forces limit; leave
#     to prepare and leave to launch given.
# These are the second version of the orders (decision 0184, "The second
# version of the orders").
#
# Kept out of the loops that walk data/scenarios/*.toml by being in this
# folder (decision 0072).
#
'''

JAPANESE = '''
[[command]]
day = 1
side = "japanese"
kind = "set_intent"
intent = 1
intent_kind = "hold"
base = 5

[[command]]
day = 1
side = "japanese"
kind = "set_limit"
limit = "forces"
intent = 1
task_forces = [5]
land_units = [27, 28, 29]
source = 2

[[command]]
day = 1
side = "japanese"
kind = "set_intent"
intent = 2
intent_kind = "neutralise"
target = 1

[[command]]
day = 1
side = "japanese"
kind = "set_limit"
limit = "forces"
intent = 2
air_groups = [27, 28]

[[command]]
day = 1
side = "japanese"
kind = "set_intent"
intent = 3
intent_kind = "operate_carriers"
task_force = 3
hex = [8, -11]

[[command]]
day = 1
side = "japanese"
kind = "set_intent"
intent = 4
intent_kind = "operate_carriers"
task_force = 4
hex = [7, -9]

[[command]]
day = 1
side = "japanese"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = 1
side = "japanese"
kind = "set_leave"
leave = "launch"
given = true
'''

ALLIED = '''
[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 1
intent_kind = "hold"
base = 1

[[command]]
day = 1
side = "allied"
kind = "set_limit"
limit = "forces"
intent = 1
land_units = [1, 2, 3, 4]
air_groups = [9, 10]

[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 2
intent_kind = "operate_carriers"
task_force = 1
hex = [6, -3]

[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 3
intent_kind = "operate_carriers"
task_force = 2
hex = [7, -3]

[[command]]
day = 1
side = "allied"
kind = "set_intent"
intent = 4
intent_kind = "deny"
hex = [3, -7]
distance = 2

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "prepare"
given = true

[[command]]
day = 1
side = "allied"
kind = "set_leave"
leave = "launch"
given = true
'''


def make(base, letter, allied, japanese):
    if base.count(NAME_OLD) != 1:
        raise SystemExit("the base's name line is not the one expected")
    if '[[command]]' in base or '[[order]]' in base:
        raise SystemExit('the base already has a command or an order')
    text = base.replace(NAME_OLD, (
        f'name = "Scenario 103, file {letter}: the Eastern Solomons over five days, '
        f'23 to 27 August 1942; {WORDS[letter]}"\n'))
    text = HEADER + text
    if not text.endswith('\n'):
        text += '\n'
    if allied:
        text += ALLIED
    if japanese:
        text += JAPANESE
    return text


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0])
    ap.add_argument('repo', nargs='?',
                    default=os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
    ap.add_argument('--check', action='store_true')
    a = ap.parse_args()
    with open(os.path.join(a.repo, BASE), encoding='utf-8') as f:
        base = f.read()
    differ = 0
    for name, letter, allied, japanese in FILES:
        text = make(base, letter, allied, japanese)
        path = os.path.join(a.repo, FOLDER, name)
        if a.check:
            try:
                with open(path, encoding='utf-8') as f:
                    kept = f.read()
            except FileNotFoundError:
                kept = None
            same = kept == text
            differ += not same
            print(f'{FOLDER}/{name}: ' + ('as made' if same else 'NOT as made: run the script without --check'))
        else:
            with open(path, 'w', encoding='utf-8') as f:
                f.write(text)
            print(f'wrote {FOLDER}/{name}')
    return 1 if differ else 0


if __name__ == '__main__':
    sys.exit(main())
